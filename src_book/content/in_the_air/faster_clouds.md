# faster Clouds

Currently the clouds really chug on my laptop. It's dropping frames at 640x480, and is ~10fps at 1080p. I reckon we can do better.


## Don't Render clouds if outside of cloud box
While we have a render sphere for draw distance, there is also a limit to how high and how low the clouds can be. There is no reason why we should bother to raymarch when outside of these bounds.



<canvas id="in_the_air/faster_clouds"></canvas>