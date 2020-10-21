# Fuzzy Search

* Contributor: [@CHNB128](https://github.com/CHNB128)

```bash
pier_fzf() {
  pier $(pier list | awk NR\>2 | fzf | awk '{print $1}')
}
```
