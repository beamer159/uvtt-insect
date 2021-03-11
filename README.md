# uvtt-insect
CLI tool for extracting and inserting UVTT images

# Usage

## Image Extraction

Extract image from UVTT `MyMap.dd2vtt`, save image with same name as UVTT, `MyMap.png`

```
uvtt-insect extract -i MyMap.dd2vtt
```

Extract image from UVTT `MyMap.dd2vtt`, save image as `CustomImageName.png`

```
uvtt-insect extract -i MyMap.dd2vtt -o CustomImageName.png
```

## Image Insertion

Insert image `MyImage.png` into UVTT `MyMap.dd2vtt`, overwriting `MyMap.dd2vtt`

```
uvtt-insect insert -i MyMap.dd2vtt -m MyImage.png
```

Insert image `CustomImageName.png` into UVTT `MyMap.dd2vtt`, save UVTT as `NewMap.dd2vtt`

```
uvtt-insect insert -i MyMap.dd2vtt -m CustomImageName.png -o NewMap.dd2vtt
```
