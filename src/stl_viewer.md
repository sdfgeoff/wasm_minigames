# STL Viewer

STL files are commonly used in 3D printing, and are easy to generate
in code. One project idea I have in mind is to use Rust to generate
files for 3D printing on the users web browser. It would be nice if
the pipeline was:
```
User Configuration --> STL --> Preview Rendering
                        |
                        V
                 Download files
```
This way you can be sure that what the user (and developer) sees is what
ends up being created.

One part of this project is to develop a renderer that takes in STL 
files and allows the user to rotate the view around them.
