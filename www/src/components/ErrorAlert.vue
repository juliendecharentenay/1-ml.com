<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <ModalDialog>
        <div class="sm:flex sm:items-start">
          <div class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
            <ExclamationIcon class="h-6 w-6 text-red-600" />
          </div>
          <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
            <h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-title">We are really sorry... An error occured:</h3>
            <div class="mt-2">
              <p class="text-sm text-gray-500" id="erroralert-p-message">{{ message }}</p>
            </div>
            <div class="mt-2 text-sm text-gray-500" v-if="error !== null">
              Error information: {{ error }}
            </div>
          </div>
        </div>
        <div class="mt-5 sm:mt-4 sm:flex sm:flex-row-reverse">
          <button type="button" class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:w-auto sm:text-sm"
                  id="erroralert-button-dismiss"
                  @click="$emit('dismiss')">Dismiss</button>
        </div>
  </ModalDialog>
</template>
<script>
import ModalDialog from "@/components/ModalDialog.vue";
import { ExclamationIcon } from '@heroicons/vue/outline';

export default {
  name: "ErrorAlert",
  components: {
    ExclamationIcon,
    ModalDialog,
  },
  props: ['message', 'error'],
  emits: ['dismiss'],
}
</script>
