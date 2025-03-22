# kind で Kubernetes Gateway API を利用する方法

## 1. Gateway API の CRD をインストール

Gateway API は標準の Kubernetes API ではないため、まず CRD をインストールする必要があります。

```bash
kubectl kustomize "https://github.com/nginxinc/nginx-gateway-fabric/config/crd/gateway-api/standard?ref=v1.5.0" | kubectl apply -f -
```

## 2. Gateway Controller をインストール

Gateway を機能させるには Gateway Controller が必要です。開発用には `nginx-gateway` などが使いやすいです。

```bash
kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.0/deploy/crds.yaml
kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.0/deploy/default/deploy.yaml
```

## 3. kind クラスタの作成（ポートマッピング付き）

```yaml
# kind-config.yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
name: gateway-test
nodes:
  - role: control-plane
    extraPortMappings:
      - containerPort: 80
        hostPort: 8080
        protocol: TCP
      - containerPort: 443
        hostPort: 8443
        protocol: TCP
```

クラスタ作成コマンド：

```bash
kind create cluster --config kind-config.yaml
```

## 4. Gateway と HTTPRoute の定義例

```yaml
apiVersion: gateway.networking.k8s.io/v1
kind: Gateway
metadata:
  name: my-gateway
  namespace: default
spec:
  gatewayClassName: nginx
  listeners:
    - name: http
      port: 80
      protocol: HTTP
      allowedRoutes:
        namespaces:
          from: All
```

```yaml
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata:
  name: my-route
spec:
  parentRefs:
    - name: my-gateway
  rules:
    - matches:
        - path:
            type: PathPrefix
            value: /
      backendRefs:
        - name: my-service
          port: 8080
```

## まとめ

| ステップ | 内容 |
|---------|------|
| ①       | Gateway API の CRD をインストール |
| ②       | Gateway Controller を導入 |
| ③       | kind クラスタにポートマッピングを追加 |
| ④       | Gateway/Route/Service を定義して接続 |
