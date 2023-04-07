#!/bin/bash
export DOCKER_BUILDKIT=1 # to make the docker build faster

if [ -z "$1" ]; then
  echo -e "\e[33mPlease choose a folder to build the image\e[0m"
  exit
fi

SERVICE=$(basename $1)
IMAGE_NAME=locally/o2gether-$SERVICE

# Change the working directory
cd $1

docker build -t $IMAGE_NAME .
# Import to minikube 
# minikube image load locally/backend-o2gether
