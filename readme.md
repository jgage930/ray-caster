# Legacy Ray Caster

The ray caster had it humble beginings casting a single ray and saving to a single ppm image.  It was later portered into sdl2.  The sdl2 code is in the /engine directory.  The sdl code will be used going forward.

The ppm rendering code has been included with several working binaries.  We must not forget where we came from.

The first iteration of our 3d ray casting did several things well, but also had a few short comings.  

#### The Good
- Create a 2d rendering of a map and implement pixel perfect wall detection.
- Create a 3d rendering from casting rays.
- Fairly easy to work with.

#### The Problems
- Choppy effect when panning view.
- Slows down when looking at things far away.
- Fixed view, no user interaction.