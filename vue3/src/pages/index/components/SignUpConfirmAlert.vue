<template>
  <ModalDialog>
    <div class="mt-3 text-left">
      <h3 class="text-lg leading-6 font-medium text-gray-900">Signup Complete</h3>
      <p class="text-gray-600">
      Your account has been created. An email has been sent to your email address '{{ email }}' with a link to 
      confirm your account. The next steps are:
      </p>
      <ul class="list-disc text-gray-600 pl-6">
        <li>Open the email title <em>[1-ml] Account Activation</em> and click on the link;</li>
        <li>Visit the webpage <a href="https://1-ml.com">https://1-ml.com</a> and signin with your username '{{ username }}';</li>
        <li>And define your chosen domain <em>prefix</em> to complete the account setup process.</li>
      </ul>
      <p class="mt-2 text-sm text-gray-600">
      <span class="text-gray-800">Can't find the email?</span> Check in the spam folder...<br/>
      If that happens or you still can't find it, get in touch at <a href="mailto:signup@support.1-ml.com">signup@support.1-ml.com</a>
      </p>
      
      <div class="mt-5 flex flex-row-reverse">
        <button @click="$emit('dismiss')"
                class="py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hove:bg-indigo-50 w-auto"
                >Dismiss</button>
      </div>
    </div>
  </ModalDialog>
</template>
<script>
import ModalDialog from '@/components/ModalDialog.vue';

export default {
  name: "SignUpConfirmAlert",
  components: {
    ModalDialog,
  },
  data: function() {
    return {};
  },
  props: ['email', 'username'],
  emits: ['dismiss']
};
</script>
