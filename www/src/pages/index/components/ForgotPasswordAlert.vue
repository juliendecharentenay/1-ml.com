<template>
  <ModalDialog>
    <div class="mt-3 text-left">
      <h3 class="text-lg leading-6 font-medium text-gray-900">Forgotten password</h3>
      <p class="mt-4-text-xl text-gray-600">Please enter your username or email address. We will email you a code to use to enter a new password:</p>
      <div class="mt-2">
        <div class="relative z-0 rounded-lg shadow">
          <div class="group relative min-w-0 overflow-hidden bg-white py-4 px-4 text-sm font-medium text-center text-gray-900 rounded-lg">
            <span v-if="step === 'send_code'">Login details</span>
            <span v-if="step === 'verify'"   >Recovery</span>
            <span aria-hidden="true" class="absolute inset-x-0 bottom-0 h-0.5 bg-indigo-500"></span>
          </div>
        </div>
        <div class="flex flex-col justify-center">
          <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
            <div class="space-y-6">
              <InputComponent v-model="username" label="Username"     type="text" v-if="step === 'send_code'" />
              <InputComponent v-model="code"     label="Code"         type="text" v-if="step === 'verify'" />
              <InputComponentPassword v-model="password" label="New password" v-if="step === 'verify'" />
            </div>
            <div class="mt-5 flex flex-row-reverse gap-2">
              <button @click="on_send_code" 
                      class="py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                      v-if="step === 'send_code'">Send Code</button>
              <button @click="on_verify" 
                      class="py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    v-if="step === 'verify'">Update password</button>
             <button @click="$emit('dismiss')"
                     class="py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hove:bg-indigo-50 w-auto"
                     >Dismiss</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </ModalDialog>
</template>
<script>
import ModalDialog from '@/components/ModalDialog.vue';
import InputComponent from '@/components/InputComponent.vue'
import InputComponentPassword from '@/components/InputComponentPassword.vue'
import { getCognitoUser, forgotPassword, confirmPassword } from '@/js/cognito.js'

export default {
  name: "ForgotPasswordAlert",
  components: {
    ModalDialog,
    InputComponent,
    InputComponentPassword,
  },
  data: function() {
    return {
      step: 'send_code',
      username: null,
      user: null,
      code: null,
      password: null,
    };
  },
  emits: ['on_error', 'on_loading', 'dismiss', 'complete'],
  methods: {
    on_send_code: function() {
      try {
        if (this.username === null) { throw new Error("Invalid username"); }
        this.user = getCognitoUser(this.username);
        this.$emit('on_loading', "Sending code");
        forgotPassword(this.user)
        .then(() => {this.$emit('on_loading', null); this.step = 'verify'; })
        .catch((e) => {this.on_error("Error in forgotPassword", e);});
      } catch (e) {
        this.on_error("Error when sending code", e);
      }
    },
    on_verify: function() {
      try {
        this.$emit('on_loading', "Verifying code and updating password");
        confirmPassword(this.user, this.code, this.password)
        .then(() => { this.$emit('on_loading', null); this.$emit('complete'); })
        .catch((e) => {this.on_error("Error when verifying code", e);});
      } catch (e) {
        this.on_error("Error when verifying code and updating password", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_loading', null);
      this.$emit('on_error', {message, error});
    }
  }
}
</script>
