<template>
  <div>
    <label for="input" class="block text-sm font-medium text-gray-700"> {{ label }} </label>
    <div class="mt-1">
      <input id="input" name="input" v-model="value" :type="type" 
       class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 sm:text-sm"
       :class="{'focus:outline-none focus:ring-indigo-500 focus:border-indigo-500': disabled === false, 'bg-gray-50': disabled === true}"
             :disabled="disabled">
    </div>
  </div>
</template>
<script>
export default {
  name: "InputComponent",
  props: ['label', 'type', 'modelValue', 'disabled'],
  emits: ['update:modelValue'],
  computed: {
    value: {
      get() { return this.modelValue; },
      set(v) { this.$emit('update:modelValue', v); },
    }
  }
};
</script>
