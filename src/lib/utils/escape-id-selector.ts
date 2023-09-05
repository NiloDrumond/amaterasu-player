export function escapeIdSelector(id: string) {
  if (!id) return id;
  if (/[0-9]/.test(id[0])) {
    return `#\\3${id[0]} ` + id.slice(1);
  }
  return '#' + id;
}
