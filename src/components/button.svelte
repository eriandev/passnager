<script lang="ts">
  import { Button } from 'bits-ui'
  import type { ButtonProps } from '@/components/types'

  let { children, class: extraClass, ...restProps }: ButtonProps = $props()
</script>

<Button.Root
  {...restProps}
  class={[
    'inline-flex h-12 cursor-pointer items-center justify-center rounded-input bg-dark px-5 text-base font-semibold text-background shadow-mini transition-all hover:bg-dark/95 active:scale-98 disabled:active:scale-100',
    extraClass,
  ]}
>
  {@render children?.()}
</Button.Root>
