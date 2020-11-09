# STL Viewer Camera Lookaround

The user should be able to use the mouse to rotate around the object.
For this we need a more than just a single matrix to use for model
transformation. So we need to introduce mat4's to our rust. Rather than 
reinvent the wheel like I did for `swoop`, I'll use glam as it seems to 
do what I want.

In fact we need three matrices: The camera transform, the object transform,
and a camera-space-to-clip-space transform. Using the one true matrix
notation these are:

 - world_to_model
 - world_to_camera
 - camera_to_screen

The STL file can contain the world_to_model matrix, but the camera
matrices should be stored elsewhere.


# The Camera Matrix
We want the camera to rotate around the center of the scene, so it makes
sense to store the camera position as an elevation and azimuth and only
do the conversion when we need the matrix.



<canvas id="stl_viewer_camera_lookaround"></canvas>

