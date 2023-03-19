<template>
<div class="relative z-10" :aria-labelledby="title" role="dialog" aria-modal="true">
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>

  <div class="fixed z-10 inset-0 overflow-y-auto">
    <div class="flex items-center justify-center min-h-full p-4 text-center sm:p-0">
      <div class="relative bg-white rounded-lg px-4 pt-5 pb-4 text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:max-w-lg sm:w-full sm:p-6">
        <slot></slot>
      </div>
    </div>
  </div>
</div>
</template>
<script>
export default {
  name: "ModalDialog",
  props: ['title'],
};
</script>
