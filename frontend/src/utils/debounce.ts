// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function debounce<T extends (...args: any[]) => void>(
  func: T,
  delay: number
): {
  (this: unknown, ...args: Parameters<T>): void;
  cancel: () => void;
} {
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  const debounced = (...args: Parameters<T>) => {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => func(...args), delay);
  };

  debounced.cancel = () => {
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
  };

  return debounced;
}

/* usage example:
  const debouncedUpdate = useRef(
    debounce((value: string) => {
      console.log('Text changed:', value);
      UpdateList(listId, value);
    }, 1000)
  ).current;

  const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(event.target.value);
    debouncedUpdate(event.target.value);    
  };
*/