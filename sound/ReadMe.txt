Rust audio libraries cpal and rodio do not work in android. So this is why libdsl is needed.
You need to have libSDL2.so
in order to include rust-sdl2
Or you can just build your own project.
1. Download latest sdl2 from https://www.libsdl.org/release/SDL2-2.0.8.tar.gz
2. tar -xzvf SDL2-2.0.8.tar.gz
3. cp -R SDL2-2.0.8/android-project ~/AndroidStudioProjects/sdl-project
4. cd ~/AndroidStudionProjects/sdl-project
5. ln -s ~/installers/SDL2-2.0.8 app/jni/SDL2
6. cd apps/jni/src
7. vi main.cpp
   #include <stdlib.h>
   #include <stdio.h>
   #include <time.h>
   #include "SDL.h"
   int main(int argc, char* argv[]){
      SDL_Window* window = 0;
      SDL_GLContext gl = 0;
   }
8. Open Android.mk and set LOCAL_SRC_FILES to 
   LOCAL_SRC_FILES := main.cpp

9. ndk-build
10. you so files are in ../../libs
11. edit ~/.cargo/config
    [target.arm-linux-androideabi.SDL2]   # refers to this line: https://github.com/Rust-SDL2/rust-sdl2/blob/master/sdl2-sys/Cargo.toml#L11
    rustc-link-search = ["/some/path"]
    rustc-link-lib = ["SDL2"]   # name of the file, with `lib` and `.so` removed

References
https://www.youtube.com/watch?v=5_we0KZqDlU
https://hg.libsdl.org/SDL/file/default/docs/README-android.md
https://github.com/tomaka/android-rs-glue/issues/135




