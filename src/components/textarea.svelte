<script lang="ts">
  import type { TextareaProps } from '@/components/types'

  let { value = $bindable(), class: extraClass, ...restProps }: TextareaProps = $props()
</script>

<textarea
  bind:value
  class={[
    'w-full rounded-card-sm border border-border-input bg-background p-2.5 text-sm placeholder:text-foreground-alt/50 hover:border-dark-40 focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background focus:outline-none',
    extraClass,
  ]}
  {...restProps}></textarea>
