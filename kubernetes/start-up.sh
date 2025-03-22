#!/usr/bin/env bash

set -e

kind create cluster --config kubernetes/dev-cluster.yaml

kubectl kustomize "https://github.com/nginxinc/nginx-gateway-fabric/config/crd/gateway-api/standard?ref=v1.5.0" | kubectl apply -f -
kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.0/deploy/crds.yaml
kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.0/deploy/default/deploy.yaml

docker build -t smart-task:latest .
kind load docker-image smart-task:latest --name smart-task

kubectl apply -k kubernetes/overlays/development --context kind-smart-task

kubectl port-forward -n nginx-gateway svc/nginx-gateway 18080:80