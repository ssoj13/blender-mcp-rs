---
area: api-types
group: camera
version: 5.2.2
---

# Blender 5.2.2 RNA types: camera

## Camera

Camera data-block for storing camera settings

- `type` · enum · {PERSP, ORTHO, PANO, CUSTOM} — Camera types
- `sensor_fit` · enum · {AUTO, HORIZONTAL, VERTICAL} — Method to fit image and field of view angle inside the sensor
- `passepartout_alpha` · float — Opacity (alpha) of the darkened overlay in Camera view
- `angle_x` · float — Camera lens horizontal field of view
- `angle_y` · float — Camera lens vertical field of view
- `angle` · float — Camera lens field of view
- `clip_start` · float — Camera near clipping distance
- `clip_end` · float — Camera far clipping distance
- `lens` · float — Perspective Camera focal length value in millimeters
- `sensor_width` · float — Horizontal size of the image sensor area in millimeters
- `sensor_height` · float — Vertical size of the image sensor area in millimeters
- `ortho_scale` · float — Orthographic Camera scale (similar to zoom)
- `display_size` · float — Apparent size of the Camera object in the 3D View
- `shift_x` · float — Camera horizontal shift
- `shift_y` · float — Camera vertical shift
- `stereo` · pointer · -> CameraStereoData · read-only
- `show_limits` · boolean — Display the clipping range and focus point on the camera
- `show_mist` · boolean — Display a line from the Camera to indicate the mist area
- `show_passepartout` · boolean — Show a darkened overlay outside the image area in Camera view
- `show_safe_areas` · boolean — Show TV title safe and action safe areas in Camera view
- `show_safe_center` · boolean — Show safe areas to fit content in a different aspect ratio
- `show_name` · boolean — Show the active Camera's name in Camera view
- `show_sensor` · boolean — Show sensor size (film gate) in Camera view
- `show_background_images` · boolean — Display reference images behind objects in the 3D View
- `lens_unit` · enum · {MILLIMETERS, FOV} — Unit to edit lens in for the user interface
- `composition_guide_color` · float · [4] — Color and alpha for compositional guide overlays
- `show_composition_center` · boolean — Display center composition guide inside the camera view
- `show_composition_center_diagonal` · boolean — Display diagonal center composition guide inside the camera view
- `show_composition_thirds` · boolean — Display rule of thirds composition guide inside the camera view
- `show_composition_golden` · boolean — Display golden ratio composition guide inside the camera view
- `show_composition_golden_tria_a` · boolean — Display golden triangle A composition guide inside the camera view
- `show_composition_golden_tria_b` · boolean — Display golden triangle B composition guide inside the camera view
- `show_composition_harmony_tri_a` · boolean — Display harmony A composition guide inside the camera view
- `show_composition_harmony_tri_b` · boolean — Display harmony B composition guide inside the camera view
- `panorama_type` · enum · {EQUIRECTANGULAR, EQUIANGULAR_CUBEMAP_FACE, MIRRORBALL, FISHEYE_EQUIDISTANT, FISHEYE_EQUISOLID, FISHEYE_LENS_POLYNOMIAL, CENTRAL_CYLINDRICAL} — Distortion to use for the calculation
- `fisheye_fov` · float — Field of view for the fisheye lens
- `fisheye_lens` · float — Lens focal length (mm)
- `latitude_min` · float — Minimum latitude (vertical angle) for the equirectangular lens
- `latitude_max` · float — Maximum latitude (vertical angle) for the equirectangular lens
- `longitude_min` · float — Minimum longitude (horizontal angle) for the equirectangular lens
- `longitude_max` · float — Maximum longitude (horizontal angle) for the equirectangular lens
- `fisheye_polynomial_k0` · float — Coefficient K0 of the lens polynomial
- `fisheye_polynomial_k1` · float — Coefficient K1 of the lens polynomial
- `fisheye_polynomial_k2` · float — Coefficient K2 of the lens polynomial
- `fisheye_polynomial_k3` · float — Coefficient K3 of the lens polynomial
- `fisheye_polynomial_k4` · float — Coefficient K4 of the lens polynomial
- `central_cylindrical_range_u_min` · float — Minimum Longitude value for the central cylindrical lens
- `central_cylindrical_range_u_max` · float — Maximum Longitude value for the central cylindrical lens
- `central_cylindrical_range_v_min` · float — Minimum Height value for the central cylindrical lens
- `central_cylindrical_range_v_max` · float — Maximum Height value for the central cylindrical lens
- `central_cylindrical_radius` · float — Radius of the virtual cylinder
- `custom_filepath` · string — Path to the shader defining the custom camera
- `custom_shader` · pointer · -> Text — Shader defining the custom camera
- `custom_mode` · enum · {INTERNAL, EXTERNAL}
- `custom_bytecode` · string — Compiled bytecode of the custom shader
- `custom_bytecode_hash` · string — Hash of the compiled bytecode of the custom shader, for quick equality checking
- `dof` · pointer · -> CameraDOFSettings · read-only
- `background_images` · collection · [] of CameraBackgroundImages · read-only — List of background images
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `cycles_custom` · pointer · -> CyclesCustomCameraSettings · read-only — Parameters for custom (OSL-based) cameras

## CameraDOFSettings

Depth of Field settings

- `use_dof` · boolean — Use Depth of Field
- `focus_object` · pointer · -> Object — Use this object to define the depth of field focal point
- `focus_subtarget` · string — Use this armature bone to define the depth of field focal point
- `focus_distance` · float — Distance to the focus point for depth of field
- `aperture_fstop` · float — F-Stop ratio (lower numbers give more defocus, higher numbers give a sharper image)
- `aperture_blades` · int — Number of blades in aperture for polygonal bokeh (at least 3)
- `aperture_rotation` · float — Rotation of blades in aperture
- `aperture_ratio` · float — Distortion to simulate anamorphic lens bokeh

## CameraStereoData

Stereoscopy settings for a Camera data-block

- `convergence_mode` · enum · {OFFAXIS, PARALLEL, TOE}
- `pivot` · enum · {LEFT, RIGHT, CENTER}
- `interocular_distance` · float — Set the distance between the eyes - the stereo plane distance / 30 should be fine
- `convergence_distance` · float — The converge point for the stereo cameras (often the distance between a projector and the projection screen)
- `use_spherical_stereo` · boolean — Render every pixel rotating the camera around the middle of the interocular distance
- `use_pole_merge` · boolean — Fade interocular distance to 0 after the given cutoff angle
- `pole_merge_angle_from` · float — Angle at which interocular distance starts to fade to 0
- `pole_merge_angle_to` · float — Angle at which interocular distance is 0

## MovieReconstructedCamera

Match-moving reconstructed camera data from tracker

- `frame` · int · read-only — Frame number marker is keyframed on
- `matrix` · float · [16] · read-only — Worldspace transformation matrix
- `average_error` · float · read-only — Average error of reconstruction

## MovieTrackingCamera

Match-moving camera data for tracking

- `distortion_model` · enum · {POLYNOMIAL, DIVISION, NUKE, BROWN} — Distortion model used for camera lenses
- `sensor_width` · float — Width of CCD sensor in millimeters
- `focal_length` · float — Camera's focal length
- `focal_length_pixels` · float — Camera's focal length
- `units` · enum · {PIXELS, MILLIMETERS} — Units used for camera focal length
- `principal_point` · float · [2] — Optical center of lens
- `principal_point_pixels` · float · [2] — Optical center of lens in pixels
- `k1` · float — First coefficient of third order polynomial radial distortion
- `k2` · float — Second coefficient of third order polynomial radial distortion
- `k3` · float — Third coefficient of third order polynomial radial distortion
- `division_k1` · float — First coefficient of second order division distortion
- `division_k2` · float — Second coefficient of second order division distortion
- `nuke_k1` · float — First coefficient of second order Nuke distortion
- `nuke_k2` · float — Second coefficient of second order Nuke distortion
- `nuke_p1` · float — First coefficient of tangential Nuke distortion
- `nuke_p2` · float — Second coefficient of tangential Nuke distortion
- `brown_k1` · float — First coefficient of fourth order Brown-Conrady radial distortion
- `brown_k2` · float — Second coefficient of fourth order Brown-Conrady radial distortion
- `brown_k3` · float — Third coefficient of fourth order Brown-Conrady radial distortion
- `brown_k4` · float — Fourth coefficient of fourth order Brown-Conrady radial distortion
- `brown_p1` · float — First coefficient of second order Brown-Conrady tangential distortion
- `brown_p2` · float — Second coefficient of second order Brown-Conrady tangential distortion
- `pixel_aspect` · float — Pixel aspect ratio
