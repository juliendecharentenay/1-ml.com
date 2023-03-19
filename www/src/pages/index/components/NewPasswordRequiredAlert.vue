<template>
  <ModalDialog>
    <div class="mt-3 text-left">
      <h3 class="text-lg leading-6 font-medium text-gray-900">Complete account setup</h3>
      <div class="mt-2 text-sm text-gray-500">Your account is set with a temporary password. Please enter a new password below:</div>
      <div class="mt-2">
        <InputComponentPassword v-model="password" label="Password" ref="input_password" 
                                @on_password_change_valid="has_valid_password = $event" />
      </div>
    </div>
    <div class="mt-5 flex flex-row-reverse gap-2">
      <button type="button" 
              class="rounded-md border border-gray-300 shadow-sm px-4 py-2 text-base font-medium text-white w-auto" 
              :class="{'bg-indigo-500 hover:bg-opacity-75': has_valid_password, 'bg-indigo-300': !has_valid_password}"
              :disabled="! has_valid_password"
              @click="set_password">Set Password</button>
      <button type="button" class="rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white      text-base font-medium text-gray-700 hover:bg-indigo-50 w-auto"
              @click="$emit('dismiss')">Dismiss</button>
    </div>
  </ModalDialog>
</template>
<script>
import ModalDialog from '@/components/ModalDialog.vue';
import InputComponentPassword from '@/components/InputComponentPassword.vue';
import { handleNewPassword } from '@/js/cognito.js';

export default {
  name: "NewPasswordRequiredAlert",
  props: ['user', 'userAttributes', 'requiredAttributes'],
  emits: ['on_error', 'on_dismiss', 'on_password_set'],
  components: {
    ModalDialog,
    InputComponentPassword,
  },
  data: function() {
    return {
      password: null,
      has_valid_password: false
    };
  },
  methods: {
    set_password: function() {
      try {
        handleNewPassword(this.user, this.userAttribues, this.password)
        .then(() => {
          this.$emit('on_password_set', this.password);
        })
        .catch((e) => {this.on_error("Error in set_password::handleNewPassword", e);});
      } catch (e) {
        this.on_error("Error in set_password", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
};
</script>
