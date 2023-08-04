use rand::Rng;

use crate::{
    gpu, Camera, CameraBuffers, CameraComputePass, CameraController, Engine,
    Params,
};

#[derive(Debug)]
pub struct DirectInitialShadingPass {
    pass: CameraComputePass<gpu::PassParams>,
}

impl DirectInitialShadingPass {
    #[allow(clippy::too_many_arguments)]
    pub fn new<P>(
        engine: &Engine<P>,
        device: &wgpu::Device,
        _: &Camera,
        buffers: &CameraBuffers,
    ) -> Self
    where
        P: Params,
    {
        let pass = CameraComputePass::builder("direct_initial_shading")
            .bind([
                &engine.noise.bind_blue_noise_texture(),
                &engine.triangles.bind_readable(),
                &engine.bvh.bind_readable(),
                &engine.lights.bind_readable(),
                &engine.materials.bind_readable(),
                &engine.images.bind_atlas_sampled(),
                &engine.world.bind_readable(),
            ])
            .bind([
                &buffers.camera.bind_readable(),
                &buffers.atmosphere_transmittance_lut.bind_sampled(),
                &buffers.atmosphere_sky_lut.bind_sampled(),
                &buffers.direct_hits.bind_readable(),
                &buffers.direct_gbuffer_d0.bind_readable(),
                &buffers.direct_gbuffer_d1.bind_readable(),
                &buffers.direct_initial_samples.bind_writable(),
            ])
            .build(device, &engine.shaders.direct_initial_shading);

        Self { pass }
    }

    pub fn run(
        &self,
        camera: &CameraController,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        // This pass uses 8x8 warps:
        let size = camera.camera.viewport.size / 8;

        let params = gpu::PassParams {
            seed: rand::thread_rng().gen(),
            frame: camera.frame,
        };

        self.pass.run(camera, encoder, size, &params);
    }
}
