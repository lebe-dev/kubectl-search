# kubectl-search

Search through config-maps and secrets (optional).

## Usage

```shell
export KUBECONFIG=~/.kube/demo.kubeconfig

alias ks=kubectl-search

# Find all configmaps and secret values which contains search mask

# ks values [OPTIONS] <namespace> <search-mask>
$ ks values --secrets apps "backup"

- config-map: app-cm
  - 'BACKUP_SRV_HOST': 'app-backup-svc' 
  
- config-map: another-app-cm
  - 'BACKUP_USER': 'app-backup-svc'
  
- secret: db-secret
  - 'BACKUP_USERNAME': '**********' # < each secret value is hidden
```

## How it works

Tool uses `kubectl` to read data from kubernetes.

## Safety

- Tool doesn't use any write or delete commands inside cluster
- Tool doesn't log or print secret values

## Roadmap

1. Mask secret values by mask: PASSWORD, TOKEN 
2. Search values in Vault secret values
3. Support cache