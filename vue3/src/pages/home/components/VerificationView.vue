<template>
  <!-- Page title -->
  <div class="border-b border-gray-200 px-4 py-4 sm:flex sm:items-center sm:justify-between sm:px-6 lg:px-8">
    <div class="flex-1 min-w-0">
      <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate"> Nearly there... We need to verify your email:</h1>
    </div>
  </div>

<!-- message to send email -->
<div class="my-4 mx-8 bg-white shadow sm:rounded-lg"
     :class="{'bg-white': step === 'get_code', 'bg-gray-50, text-gray-300': step !== 'get_code'}"
  >
  <div class="mx-auto max-w-2xl px-4 py-5 sm:p-6">
    <h3 class="text-lg leading-6 font-medium"
        :class="{'text-gray-900': step === 'get_code'}"
        >Get a confirmation code</h3>
    <div class="mt-2 max-w-xl text-sm"
        :class="{'text-gray-500': step === 'get_code'}"
        >
      <p>Press the <em>Get Code</em> button below. This will send a confirmation code to your email {{ email }} that you
         can enter in the text code below to confirm ownership of the email address.</p>
      <p>If you already have a code, click on <em>I have a code</em> and enter the code.</p>
    </div>
    <div class="mt-5 flex flex-row gap-4 items-center">
      <button type="submit" class="w-full inline-flex items-center justify-center px-4 py-2 border border-transparent shadow-sm font-medium rounded-md text-white sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
              :class="{'bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500': step === 'get_code', 'bg-indigo-300': step !== 'get_code'}"
              :disabled="step !== 'get_code'"
              @click="get_code();">Get code</button>
      <a href="#!" @click="step = 'confirm_code';"
         class="font-medium"
         :class="{'text-indigo-600 hover:text-indigo-500': step === 'get_code', 'text-indigo-300': step !== 'get_code'}"
         >I have a code</a>
    </div>
  </div>
</div>

<!-- Enter confirmation code -->
<div class="my-4 mx-8 bg-white shadow sm:rounded-lg"
     :class="{'bg-white': step === 'confirm_code', 'bg-gray-50, text-gray-300': step !== 'confirm_code'}"
   >
  <div class="mx-auto max-w-2xl px-4 py-5 sm:p-6">
    <h3 class="text-lg leading-6 font-medium"
        :class="{'text-gray-900': step === 'confirm_code'}"
        >Enter the confirmation code</h3>
    <div class="mt-2 max-w-xl text-sm"
        :class="{'text-gray-500': step === 'confirm_code'}"
         >
      <p>Please enter the confirmation code that has been sent to your email address {{ email }}. It may take 
         a couple of minutes - and may be in your spam/junk email.</p>
    </div>
    <form class="mt-5 sm:flex sm:items-center">
      <div class="w-full sm:max-w-xs">
        <label for="code" class="sr-only">Confirmation Code</label>
        <input type="text" v-model="code" name="code" 
               class="shadow-sm block w-full sm:text-sm border-gray-300 rounded-md" 
               :class="{'focus:ring-indigo-500 focus:border-indigo-500': step === 'confirm_code'}"
               :disabled="step !== 'confirm_code'"
               >
      </div>
      <button type="submit" class="mt-3 w-full inline-flex items-center justify-center px-4 py-2 border border-transparent shadow-sm font-medium rounded-md text-white bg-indigo-600 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
              :class="{'bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500': step === 'confirm_code', 'bg-indigo-300': step !== 'confirm_code'}"
              @click="confirm_code(code);"
              :disabled="step !== 'confirm_code'"
              >Confirm</button>
    </form>
  </div>
</div>
</template>

<script>
import { getAttributeVerificationCode, verifyAttribute } from "@/js/cognito.js";

export default {
  name: "VerificationView",
  props: ["user", "email"],
  emits: ["loading", "on_error", "on_confirmed"],
  data: function() {
    return {
      step: 'get_code',
      code: null,
    };
  },
  mounted: function() {
    try {
      if (this.email === undefined) {throw new Error("User email is undefined");}
    } catch (e) {
      this.on_error("Error in verification", e);
    }
  },
  methods: {
    get_code: function() {
      try {
        this.$emit('loading', 'Sending registration code');
        getAttributeVerificationCode(this.user, 'email')
        .then(() => { this.$emit('loading', null); this.step = 'confirm_code'; })
        .catch((e) => {this.on_error("Error when sending registration code", e);});
      } catch (e) {
        this.on_error("Error whilst sending code", e);
      }
    },
    confirm_code: function(code) {
      try {
        this.$emit('loading', 'Confirming registration code');
        verifyAttribute(this.user, 'email', code)
        .then(() => { this.$emit('loading', null); this.$emit('on_confirmed', null); })
        .catch((e) => {this.on_error("Error when confirming registration code", e);});
      } catch (e) {
        this.on_error("Error whilst confirming code", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', { message, error });
    }
  }
};
</script>
