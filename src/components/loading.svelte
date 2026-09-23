<script lang="ts">
  import { Motion } from 'svelte-motion'
  import LockKey from 'phosphor-svelte/lib/LockKeyIcon'
</script>

<Motion let:motion initial={{ opacity: 0, scale: 0.95 }} animate={{ opacity: 1, scale: 1 }}>
  <div use:motion class="rounded-2xl p-8">
    <div class="text-center">
      <div class="relative size-24 mx-auto">
        <Motion let:motion animate={{ rotate: 360 }} transition={{ duration: 3, repeat: Infinity, ease: 'linear' }}>
          <div
            use:motion
            class="absolute inset-0 rounded-full border-4 border-transparent border-t-primary border-r-primary/50"
          >
            <!-- -->
          </div>
        </Motion>
        <Motion
          let:motion
          animate={{ scale: [1, 1.1, 1], opacity: [0.5, 1, 0.5] }}
          transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
        >
          <div use:motion class="absolute inset-2 rounded-full border-2 border-secondary/40"><!-- --></div>
        </Motion>
        <Motion let:motion animate={{ rotate: -360 }} transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}>
          <div
            use:motion
            class="absolute inset-4 rounded-full border-4 border-transparent border-b-secondary border-l-secondary/50"
          >
            <!-- -->
          </div>
        </Motion>
        <Motion
          let:motion
          animate={{ scale: [0.8, 1, 0.8] }}
          transition={{ duration: 1.5, repeat: Infinity, ease: 'easeInOut' }}
        >
          <div
            use:motion
            class="absolute inset-6 rounded-full bg-linear-to-br from-primary to-secondary flex items-center justify-center"
          >
            <Motion
              animate={{ rotate: [0, 10, -10, 0] }}
              transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
            >
              <LockKey class="size-6" />
            </Motion>
          </div>
        </Motion>
      </div>
    </div>
  </div>
</Motion>
