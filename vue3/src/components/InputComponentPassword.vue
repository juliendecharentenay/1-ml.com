<template>
  <InputComponent :label="label" type="password" v-model="value" :disabled="disabled" />
  <p class="mt-2 text-sm text-red-600" id="inputcomponentpassword-p-require" v-if="requirements(this.value).length > 0">Require: {{ requirements().join(", ") }}</p>
</template>
<script>
import InputComponent from "@/components/InputComponent.vue";

export default {
  name: "InputComponentPassword",
  props: ['label', 'modelValue', 'disabled'],
  emits: ['update:modelValue', 'on_password_change_valid'],
  components: {
    InputComponent,
  },
  computed: {
    value: {
      get() { return this.modelValue; },
      set(v) { this.$emit('update:modelValue', v); this.$emit('on_password_change_valid', this.requirements(v).length === 0);},
    },
  },
  methods: {
    requirements: function(password) {
      let requirements = [];
      if (/.{8}/.test(password) === false) {requirements.push("8 characters");}
      if (/\d/.test(password) === false) {requirements.push("1 number");}
      if (/\(|\^|\$|\*|\.|\[|\]|\{|\}|\?|-|!|@|#|%|&|\/|\\|,|>|<|:|;|\||_|~|\+|=/.test(password) === false) {requirements.push("1 special character");}
      if (/[a-z]/.test(password) === false) {requirements.push("1 lowercase letter");}
      if (/[A-Z]/.test(password) === false) {requirements.push("1 uppercase letter");}
      return requirements;
/*
      if (requirements.length > 0) {
        return `Require: ${requirements.join(", ")}`;
      } else {
        return '';
      }
*/
    }
  },
};
</script>
