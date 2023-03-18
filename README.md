
# Setup

https://crates.io/crates/opencv

## OpenCV

https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html
https://docs.opencv.org/4.x/db/d05/tutorial_config_reference.html

Read this page and follow the steps described in "Build with opencv_contrib", knowing that the `configure` step just before the `cmake --build .` must be tweaked like this :

`cmake -DWITH_GTK=ON -DWITH_QUIRC=ON -DCMAKE_INSTALL_PREFIX=$HOME/.local -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.x/modules ../opencv-4.x`


`cmake -DWITH_QUIRC=ON -DCMAKE_INSTALL_PREFIX=$HOME/.local -DBUILD_QUIRC=ON -DQUIRC=ON -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.x/modules ../opencv-4.x`


This is the `-DBUILD_QUIRC=ON -DQUIRC=ON` that matter here to avoid the below error message when running the `cargo build`

`Library QUIRC is not linked. No decoding is performed. Take it to the OpenCV repository`

Don't forget to `sudo make install` after running the `cmake --build .` command

export OpenCV_DIR=/home/francois/dev/rust/opencv-projects/opencv-4.x/cmake
export LD_LIBRARY_PATH=/home/francois/.local/lib

sudo apt-get install libgtk2.0-dev

