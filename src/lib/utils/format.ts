export function formatDuration(seconds: number) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(
      2,
      '0',
    )}:${String(remainingSeconds).padStart(2, '0')}`;
  } else {
    return `${String(minutes).padStart(2, '0')}:${String(
      remainingSeconds,
    ).padStart(2, '0')}`;
  }
}

export function formatFileSize(sizeInBytes: number) {
  if (sizeInBytes < 1024) {
    return `${sizeInBytes.toFixed(2)} B`;
  } else if (sizeInBytes < 1048576) {
    const sizeInKB = sizeInBytes / 1024;
    return `${sizeInKB.toFixed(2)} KB`;
  } else if (sizeInBytes < 1073741824) {
    const sizeInMB = sizeInBytes / 1048576;
    return `${sizeInMB.toFixed(2)} MB`;
  } else {
    const sizeInGB = sizeInBytes / 1073741824;
    return `${sizeInGB.toFixed(2)} GB`;
  }
}
