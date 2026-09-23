<script lang="ts">
  import type { InputProps } from '@/components/types'

  let { value = $bindable(), leftIcon, rightIcon, class: extraClass, ...restProps }: InputProps = $props()
</script>

<label
  class={[
    'flex w-full gap-x-2.25 min-h-input items-center rounded-card-sm py-1 px-2.5 border border-border-input bg-background',
    extraClass,
  ]}
>
  {@render leftIcon?.()}
  <input
    bind:value
    class={[
      'inline-flex h-full w-full text-sm placeholder:text-foreground-alt/50 hover:border-dark-40 focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background focus:outline-none',
    ]}
    {...restProps}
  />
  {@render rightIcon?.()}
</label>
