<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <header class="bg-indigo-900">
    <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 border-b border-indigo-400 border-opacity-25" aria-label="Top">
      <div class="w-full py-6 flex items-center justify-between border-b border-indigo-500 lg:border-none">
        <div class="flex items-center">
          <a href="#" class="relative text-white">
            <span class="sr-only">Home</span>
            <EnvelopeIcon class="h-10 w-10" />
          </a>
          <div class="hidden ml-10 space-x-8 lg:block">
            <a v-for="i in navigation"
               :key="i.emit"
               href="#!" @click="$emit(i.emit)"         
               :id="`headerelement-a-${i.emit}`"
               class="text-base font-medium text-white hover:text-indigo-50"> {{ i.label }} </a>
          </div>
        </div>
        <div class="ml-10 space-x-4">
          <a href="#!" @click="$emit('to_signup')" class="inline-block bg-indigo-500 py-2 px-4 border border-transparent rounded-md text-base font-medium text-white hover:bg-opacity-75" id='headerelement-a-to_signup'>Sign up</a>
          <a href="#!" @click="$emit('to_signin')" class="inline-block bg-white      py-2 px-4 border border-transparent rounded-md text-base font-medium text-indigo-600 hover:bg-indigo-50" id='headerelement-a-to_signin'>Sign in</a>
        </div>
      </div>
      <div class="py-4 flex flex-wrap justify-center space-x-6 lg:hidden">
        <a v-for="i in navigation"
           :key="i.emit"
           :id="`headerelement-a-${i.emit}`"
           href="#!" @click="$emit(i.emit)"         
           class="text-base font-medium text-white hover:text-indigo-50"> {{ i.label }} </a>
      </div>
    </nav>
  </header>
</template>

<script>
import { EnvelopeIcon } from "@heroicons/vue/24/solid";

export default {
  name: "HeaderElement",
  data: function() {
    return {
      navigation: [
        { label: 'Why?',        emit:'to_why' },
        { label: 'Get Started', emit:'to_get_started' },
        { label: 'Pricing',     emit:'to_pricing'     },
        { label: 'Terms',       emit:'to_terms'       },
        { label: 'Privacy',     emit:'to_privacy'     },
      ],
    };
  },
  components: {
    EnvelopeIcon,
  },
}
</script>
