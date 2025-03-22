#!/usr/bin/env bash

set -e

kubectl delete -k kubernetes/overlays/development --context kind-smart-task
kind delete cluster --name smart-task