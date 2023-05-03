## Updating the Classifier enum

The values of the `Classifier` are automatically pulled from the canonical python
package using a build script. The values in the enum, as well as the corresponding
`PYPA_VERSION` should *_only_* be modified by running this script.

To pull in updates from trove-classifiers:
```
cd src/
python -m pip install trove-classifiers==$NEW_TROVE_VERSION
python build.py
```
