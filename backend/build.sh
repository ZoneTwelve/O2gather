#!/bin/bash
CNAME=locally/backend-o2gether
docker build -t $CNAME .
# Import to minikube 
# minikube image load locally/backend-o2gether
