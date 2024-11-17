<template>
  <div aria-live="assertive" class="fixed inset-0 flex items-end px-4 py-6 pointer-events-none sm:p-6">
    <div class="w-full flex flex-col items-center md:items-end"
         :class="{'opacity-0': messages.length === 0, 'opacity-100': messages.length > 0}">
      <div class="max-w-sm w-full bg-gray-50 border border-gray-200 shadow-lg rounded-lg pointer-events-auto ring-1 ring-black ring-opacity-5 overflow-hidden">
        <div class="p-4">
          <div class="flex flex-col gap-y-1">
            <p v-for="(message, index) in messages"
               :key="index"
               class="text-sm text-center text-gray-500">{{ message }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  name: "SnackAlert",
  emits: ['on_error'],
  data: function() {
    return {
      messages: []
    };
  },
  methods: {
    display: function(text) {
      try {
        this.messages.push(text); setTimeout(this.clear_first, 3000);
      } catch (e) {
        this.on_error("Error in display", e);
      }
    },
    clear_first: function() {
      try {
        this.messages.shift();
      } catch (e) {
        this.on_error("Error in clear_first", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', { message, error });
      this.messages = [];
    }
  }
};
</script>
