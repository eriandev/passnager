<script lang="ts">
  import { Button } from 'bits-ui'
  import type { ButtonProps } from '@/components/types'

  let { children, class: extraClass, variant = 'primary', ...restProps }: ButtonProps = $props()
</script>

<Button.Root {...restProps} class={['btn', variant, extraClass]}>
  {@render children?.()}
</Button.Root>
