<script lang="ts">
  import { Select } from 'bits-ui'
  import Check from 'phosphor-svelte/lib/CheckIcon'
  import CaretUpDown from 'phosphor-svelte/lib/CaretUpDownIcon'
  import CaretDoubleUp from 'phosphor-svelte/lib/CaretDoubleUpIcon'
  import CaretDoubleDown from 'phosphor-svelte/lib/CaretDoubleDownIcon'
  import type { SelectProps } from '@/components/types'

  let { id, placeholder, items = [], icon, value = null, onValueChange }: SelectProps = $props()

  const selectedLabel = $derived(value ? items.find((option) => option.value === value)?.label : placeholder)

  // bits-ui's single select has exactly two states: a string, or nothing, and it
  // spells "nothing" differently depending on where you look — `''` in the value
  // it is given, `undefined` in the change it reports. Both mean the same thing to
  // the app, so the falsy check is on the value rather than an equality against
  // one spelling of it.
  function handleChange(v: string) {
    onValueChange(v ? v : null)
  }
</script>

<Select.Root value={value ?? ''} {items} type="single" onValueChange={handleChange}>
  <Select.Trigger
    {id}
    aria-label={id ?? placeholder}
    class="inline-flex h-input w-full items-center rounded-9px border border-border-input bg-background px-2.75 text-sm transition-colors placeholder:text-foreground-alt/50 focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background focus:outline-none"
  >
    {@render icon?.()}
    {selectedLabel}
    <CaretUpDown class="ml-auto size-6 text-muted-foreground" />
  </Select.Trigger>

  <Select.Portal>
    <Select.Content
      {id}
      sideOffset={10}
      class="focus-override z-50 max-h-(--bits-select-content-available-height) w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) rounded-xl border border-muted bg-background px-1 py-3 shadow-popover outline-hidden select-none data-[side=bottom]:translate-y-1 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:-translate-x-1 data-[side=left]:slide-in-from-right-2 data-[side=right]:translate-x-1 data-[side=right]:slide-in-from-left-2 data-[side=top]:-translate-y-1 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95"
    >
      <Select.ScrollUpButton class="flex w-full items-center justify-center">
        <CaretDoubleUp class="size-3" />
      </Select.ScrollUpButton>
      <Select.Viewport class="p-1">
        {#each items as { label, value, disabled } (value)}
          <Select.Item
            {value}
            {label}
            {disabled}
            class="flex h-10 w-full items-center justify-between rounded-button py-3 pr-1.5 pl-5 text-sm transition-all duration-75 outline-none select-none data-highlighted:bg-muted"
          >
            {#snippet children({ selected })}
              {label}
              {#if selected}
                <Check aria-label="check" />
              {/if}
            {/snippet}
          </Select.Item>
        {/each}
      </Select.Viewport>
      <Select.ScrollDownButton class="flex w-full items-center justify-center">
        <CaretDoubleDown class="size-3" />
      </Select.ScrollDownButton>
    </Select.Content>
  </Select.Portal>
</Select.Root>
