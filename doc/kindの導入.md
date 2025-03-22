

 # kind を使ったローカル Kubernetes クラスタの導入方法
 
 ## 1. kind のインストール
 
 まず、`kind` をローカルにインストールします。
 
 ### Homebrew (macOS)
 
 ```bash
 brew install kind
 ```
 
 ## 2. クラスタ設定ファイルの作成
 
 以下のような設定ファイル（例: `kind-cluster.yaml`）を作成します。
 
 ```yaml
 kind: Cluster
 apiVersion: kind.x-k8s.io/v1alpha4
 name: smart-task
 nodes:
   - role: control-plane
     extraPortMappings:
       - containerPort: 80
         hostPort: 18080
         protocol: TCP
 ```
 
 この設定では、Kubernetes クラスタ内の `80` 番ポートが、ローカル PC の `18080` 番ポートにマッピングされます。
 
 ## 3. クラスタの作成
 
 ```bash
 kind create cluster --config kind-cluster.yaml
 ```
 
 ## 4. Docker イメージのロード（任意）
 
 ローカルでビルドした Docker イメージを kind クラスタに取り込むには以下を実行します。
 
 ```bash
 kind load docker-image your-image:tag --name smart-task
 ```
 
 ## 5. リソースの適用
 
 kustomize またはマニフェストを使ってリソースをデプロイします。
 
 ```bash
 kubectl apply -k kubernetes/overlays/development
 ```
 
 ## 6. 動作確認
 
 必要に応じてポートフォワードを使ってアクセスできます。
 
 ```bash
 kubectl port-forward -n nginx-gateway svc/nginx-gateway 18080:80
 curl http://localhost:18080/
 ```
 
 ---
 
 Cloudflare Tunnel や他の外部公開手段と組み合わせる場合は、NodePort や hostNetwork の設定も活用してください。