## Pre-requisites:
You need to have PyTorch C++ installed, for this follow go to [this link](https://pytorch.org/get-started/locally/).

To use CUDA you must have to install CUDA then run the lines below to export Torch lib:
```
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

Additionaly to run this project you need to have these packages installed:
```sh
sudo apt-get install libfontconfig1-dev
sudo apt install libssl-dev
```
