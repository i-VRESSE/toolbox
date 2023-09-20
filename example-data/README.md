# Example-data

Uncompress the example data

```bash
# compress the example data
tar -cf - run1-ranairCDR-test | xz -9ze -T0 >example-data.txz
# uncompress the example data
xz -d -c example-data.txz | tar -xf -
```
