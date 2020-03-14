#![windows_subsystem = "windows"]

use cpp_core::{NullPtr, Ptr};
use qt_3d_core::{QEntity, QNode, QTransform};
use qt_3d_extras::{
    QMetalRoughMaterial, QOrbitCameraController, QPlaneMesh, QSphereMesh, Qt3DWindow,
};
use qt_3d_input::{QKeyboardDevice, QKeyboardHandler};
use qt_3d_render::{QDirectionalLight, QPointLight, QSpotLight};
use qt_core::{QPtr, SlotNoArgs};
use qt_gui::{
    q_surface_format::OpenGLContextProfile, QColor, QGuiApplication, QSurfaceFormat, QVector3D,
};

struct Scene {
    lights: Vec<QPtr<QNode>>,
    active_index: usize,
}

fn setup_scene(root: Ptr<QEntity>) -> Scene {
    unsafe {
        let scene = QEntity::new_1a(root);

        let sphere1 = QEntity::new_1a(&scene);

        let sphere1_mesh = QSphereMesh::new_0a();
        sphere1_mesh.set_radius(1.0);
        sphere1_mesh.set_rings(60);
        sphere1_mesh.set_slices(30);
        sphere1.add_component(&sphere1_mesh);

        let sphere1_material = QMetalRoughMaterial::new_0a();
        sphere1_material.set_base_color_q_color(&QColor::from_rgb_3a(255, 255, 255));
        sphere1_material.set_metalness_float(0.5);
        sphere1_material.set_roughness_float(0.2);
        sphere1.add_component(&sphere1_material);

        let sphere1_transform = QTransform::new_0a();
        sphere1_transform.set_translation(&QVector3D::from_3_float(-2.0, 0.0, 0.0));
        sphere1.add_component(&sphere1_transform);

        let sphere2 = QEntity::new_1a(&scene);

        let sphere2_mesh = QSphereMesh::new_0a();
        sphere2_mesh.set_radius(1.0);
        sphere2_mesh.set_rings(60);
        sphere2_mesh.set_slices(30);
        sphere2.add_component(&sphere2_mesh);

        let sphere2_material = QMetalRoughMaterial::new_0a();
        sphere2_material.set_base_color_q_color(&QColor::from_rgb_3a(255, 255, 255));
        sphere2_material.set_metalness_float(0.5);
        sphere2_material.set_roughness_float(0.2);
        sphere2.add_component(&sphere2_material);

        let sphere2_transform = QTransform::new_0a();
        sphere2_transform.set_translation(&QVector3D::from_3_float(2.0, 0.0, 0.0));
        sphere2.add_component(&sphere2_transform);

        let plane = QEntity::new_1a(&scene);

        plane.add_component(&QPlaneMesh::new_0a());

        let plane_material = QMetalRoughMaterial::new_0a();
        plane_material.set_base_color_q_color(&QColor::from_rgb_3a(255, 255, 255));
        plane_material.set_metalness_float(0.5);
        plane_material.set_roughness_float(0.5);
        plane.add_component(&plane_material);

        let plane_transform = QTransform::new_0a();
        plane_transform.set_scale(100.0);
        plane_transform.set_translation(&QVector3D::from_3_float(0.0, -2.0, 0.0));
        plane.add_component(&plane_transform);

        let directional_light = QDirectionalLight::new_0a();
        directional_light.set_enabled(true);
        directional_light.set_color(&QColor::from_rgb_3a(255, 0, 0));
        directional_light.set_intensity(1.0);
        directional_light.set_world_direction(&QVector3D::from_3_float(1.0, -1.0, 0.0));
        scene.add_component(&directional_light);

        let point_light_entity = QEntity::new_1a(&scene);

        let point_light = QPointLight::new_0a();
        point_light.set_enabled(false);
        point_light.set_color(&QColor::from_rgb_3a(0, 255, 0));
        point_light.set_intensity(1.0);
        point_light.set_linear_attenuation(0.01);
        point_light.set_quadratic_attenuation(0.05);
        point_light_entity.add_component(&point_light);

        let point_light_transform = QTransform::new_0a();
        point_light_transform.set_translation(&QVector3D::from_3_float(0.0, 3.0, 1.0));
        point_light_entity.add_component(&point_light_transform);

        let spot_light_entity = QEntity::new_1a(&scene);

        let spot_light = QSpotLight::new_0a();
        spot_light.set_enabled(false);
        spot_light.set_color(&QColor::from_rgb_3a(0, 0, 255));
        spot_light.set_intensity(1.0);
        spot_light.set_local_direction(&QVector3D::from_3_float(-1.0, -1.0, 0.0));
        spot_light.set_cut_off_angle(45.0);
        spot_light.set_linear_attenuation(0.05);
        spot_light.set_quadratic_attenuation(0.005);
        spot_light_entity.add_component(&spot_light);

        let spot_light_transform = QTransform::new_0a();
        spot_light_transform.set_translation(&QVector3D::from_3_float(6.0, 6.0, 0.0));
        spot_light_entity.add_component(&spot_light_transform);

        Scene {
            active_index: 0,
            lights: vec![
                directional_light.static_upcast(),
                point_light.static_upcast(),
                spot_light.static_upcast(),
            ],
        }
    }
}

fn main() {
    unsafe {
        let format = QSurfaceFormat::new_0a();
        format.set_version(3, 3);
        format.set_profile(OpenGLContextProfile::CoreProfile);
        format.set_depth_buffer_size(24);
        format.set_samples(4);
        format.set_stencil_buffer_size(8);
        QSurfaceFormat::set_default_format(&format);

        QGuiApplication::init(|_| {
            let window = Qt3DWindow::new_0a();

            let root = QEntity::new_0a();
            let mut lights = setup_scene(root.as_ptr());

            let next_light_slot = SlotNoArgs::new(NullPtr, move || {
                lights.active_index = (lights.active_index + 1) % lights.lights.len();
                for (index, light) in lights.lights.iter().enumerate() {
                    light.set_enabled(index == lights.active_index);
                }
            });

            let keyboard_device = QKeyboardDevice::new_1a(&root);
            let handler = QEntity::new_1a(&root);
            let keyboard_handler = QKeyboardHandler::new_0a();
            keyboard_handler.set_source_device(&keyboard_device);
            keyboard_handler.set_focus(true);
            keyboard_handler.tab_pressed().connect(&next_light_slot);
            keyboard_handler.space_pressed().connect(&next_light_slot);
            handler.add_component(&keyboard_handler);

            let camera = window.camera();
            camera.set_position(&QVector3D::from_3_float(0.0, 0.0, 30.0));
            camera.set_view_center(&QVector3D::from_3_float(0.0, 0.0, 0.0));

            let controller = QOrbitCameraController::new_1a(&root);
            controller.set_camera(camera);
            controller.set_linear_speed(50.0);
            controller.set_look_speed(180.0);

            window.set_root_entity(&root);
            window.show();
            QGuiApplication::exec()
        })
    }
}
