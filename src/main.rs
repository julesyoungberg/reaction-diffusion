use nannou::prelude::*;

mod capture;
mod render;
mod uniforms;
mod util;

use crate::capture::*;
use crate::render::*;
use crate::uniforms::*;
use crate::util::*;

struct Model {
    uniform_texture: wgpu::Texture,
    updater: CustomRenderer,
    render: CustomRenderer,
    uniforms: UniformBuffer,
    capturer: FrameCapturer,
}

const WIDTH: u32 = 1440;
const HEIGHT: u32 = 810;
const SPEED: u32 = 10;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let device = window.swap_chain_device();

    let uniforms = UniformBuffer::new(device, WIDTH as f32, HEIGHT as f32, app.time);

    // Create the compute shader module.
    println!("loading shaders");
    let vs_mod = compile_shader(app, device, "shader.vert", shaderc::ShaderKind::Vertex);
    let init_fs_mod = compile_shader(app, device, "init.frag", shaderc::ShaderKind::Fragment);
    let update_fs_mod = compile_shader(app, device, "update.frag", shaderc::ShaderKind::Fragment);
    let draw_fs_mod = compile_shader(app, device, "shader.frag", shaderc::ShaderKind::Fragment);

    // create our custom texture for rendering
    println!("creating app texure");
    let sample_count = window.msaa_samples();
    let size = pt2(WIDTH as f32, HEIGHT as f32);

    println!("creating uniform texture");
    let uniform_texture = create_uniform_texture(&device, size, sample_count);

    // Create the sampler for sampling from the source texture.
    println!("creating sampler");
    let sampler = wgpu::SamplerBuilder::new().build(device);

    let init = CustomRenderer::new::<Uniforms>(
        device,
        &vs_mod,
        &init_fs_mod,
        None,
        None,
        None,
        None,
        Some(&uniforms.buffer),
        WIDTH,
        HEIGHT,
        sample_count,
    )
    .unwrap();

    let updater = CustomRenderer::new::<Uniforms>(
        device,
        &vs_mod,
        &update_fs_mod,
        None,
        None,
        Some(&vec![&uniform_texture]),
        Some(&sampler),
        Some(&uniforms.buffer),
        WIDTH,
        HEIGHT,
        sample_count,
    )
    .unwrap();

    let render = CustomRenderer::new::<Uniforms>(
        device,
        &vs_mod,
        &draw_fs_mod,
        None,
        None,
        Some(&vec![&uniform_texture]),
        Some(&sampler),
        None,
        WIDTH,
        HEIGHT,
        sample_count,
    )
    .unwrap();

    // Create our `Draw` instance and a renderer for it.
    let mut capturer = FrameCapturer::new(app);

    // Render our drawing to the texture.
    println!("rendering");
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture-renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);

    // draw initial aggregate
    println!("drawing initial design");
    init.render(&mut encoder);

    init.texture_reshaper
        .encode_render_pass(&render.output_texture.view().build(), &mut encoder);

    // copy app texture to uniform texture
    println!("copying app texture to buffer");
    render
        .texture_reshaper
        .encode_render_pass(&uniform_texture.view().build(), &mut encoder);

    capturer.take_snapshot(device, &mut encoder, &render.output_texture);

    // submit encoded command buffer
    println!("submitting encoded command buffer");
    window.swap_chain_queue().submit(&[encoder.finish()]);

    capturer.save_frame(app);

    Model {
        uniform_texture,
        updater,
        render,
        uniforms,
        capturer,
    }
}

fn diffuse(model: &mut Model, encoder: &mut wgpu::CommandEncoder) {
    model.updater.render(encoder);
    model
        .updater
        .texture_reshaper
        .encode_render_pass(&model.uniform_texture.view().build(), encoder);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window = app.main_window();
    let device = window.swap_chain_device();

    // The encoder we'll use to encode the compute pass and render pass.
    let desc = wgpu::CommandEncoderDescriptor {
        label: Some("encoder"),
    };
    let mut encoder = device.create_command_encoder(&desc);

    model.uniforms.update(device, &mut encoder, app.time);

    for _ in 0..SPEED {
        diffuse(model, &mut encoder);
    }

    model.render.render(&mut encoder);

    // copy app texture to uniform texture
    model
        .updater
        .texture_reshaper
        .encode_render_pass(&model.uniform_texture.view().build(), &mut encoder);

    model
        .capturer
        .take_snapshot(device, &mut encoder, &model.render.output_texture);

    // submit encoded command buffer
    window.swap_chain_queue().submit(&[encoder.finish()]);

    model.capturer.save_frame(app);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model
        .render
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

fn create_uniform_texture(device: &wgpu::Device, size: Point2, msaa_samples: u32) -> wgpu::Texture {
    wgpu::TextureBuilder::new()
        .size([size[0] as u32, size[1] as u32])
        .usage(
            wgpu::TextureBuilder::REQUIRED_IMAGE_TEXTURE_USAGE
                | wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        )
        .sample_count(msaa_samples)
        .format(Frame::TEXTURE_FORMAT)
        .build(device)
}
