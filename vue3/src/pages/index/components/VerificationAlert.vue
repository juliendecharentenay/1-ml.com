<template>
  <ModalDialog>
    <div class="mt-3 text-left">
      <h3 class="text-lg leading-6 font-medium text-gray-900">Account Verification</h3>
      <div class="mt-2 text-sm text-gray-500">To verify your account, please enter your account username and the verification code that has been sent to your email:</div>
      <div class="mt-2 space-y-6">
        <InputComponent v-model="l_username"  label="Username" type="text" />
        <InputComponent v-model="l_code"      label="Verification Code" type="text" />
      </div>
    </div>
    <div class="mt-5 flex flex-row-reverse gap-2">
      <button type="button" 
              class="rounded-md border border-gray-300 shadow-sm px-4 py-2 text-base font-medium text-white w-auto" 
              :class="{'bg-indigo-500 hover:bg-opacity-75': has_valid_input, 'bg-indigo-300': !has_valid_input}"
              :disabled="! has_valid_input"
              @click="verify">Verify</button>
      <button type="button" class="rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white      text-base font-medium text-gray-700 hover:bg-indigo-50 w-auto"
              @click="$emit('dismiss')">Dismiss</button>
    </div>
  </ModalDialog>
</template>
<script>
import ModalDialog from '@/components/ModalDialog.vue';
import InputComponent from '@/components/InputComponent.vue';
import { confirmCognitoUser } from '@/js/cognito.js';

export default {
  name: "VerificationAlert",
  props: ['code', 'username'],
  emits: ['on_error', 'dismiss', 'on_verified', 'username', 'code'],
  components: {
    ModalDialog,
    InputComponent,
  },
  data: function() {
    return {};
  },
  computed: {
    has_valid_input: function() {
      return this.l_username && this.l_code;
    },
    l_username: {
      get: function() { return this.username; },
      set: function(v) { this.$emit('username', v); },
    },
    l_code: {
      get: function() { return this.code; },
      set: function(v) { this.$emit('code', v); },
    }
  },
  methods: {
    verify: function() {
      try {
        confirmCognitoUser(this.l_username, this.l_code)
        .then((r) => {
console.log(r);
          this.$emit('on_verified');
        })
        .catch((e) => {this.on_error("Error in verify::confirmCognitoUser", e);});
      } catch (e) {
        this.on_error("Error in verify", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
};
</script>
