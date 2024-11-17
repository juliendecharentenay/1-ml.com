<template>
  <div class="py-12 sm:py-16 lg:py-20">
    <div class="px-6 max-w-5xl mx-auto">
      <div class="text-left text-lg text-gray-500">
        <h2 class="text-4xl font-bold text-gray-900">
        Terms of Service
        </h2>

        <dl class="mt-12 space-y-10 sm:space-y-0 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-12 lg:grid-cols-3 lg:gap-x-8">
          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Purpose</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            The service provides you a number of email addresses that you can give for others' to send you emails, hence
            masking your own email. The email addresses can not be used to send emails to others.
            </dd>
          </div>

          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Legal</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            Using the service for illegal purposes is prohibited.<br/>
            Similary, using the service to request multiple free trials, accounts or generally abuse other services or 
            users' trust is frowned upon.
            </dd>
          </div>

          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Warranty</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            We will endeavour to forward emails, but are not in a position to guarantee it. Do not use the service
            for critical email communication - ie receiving an airline ticket if you are unable to request the email to be
            resent.
            </dd>
          </div>

          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Security</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            Do not use for critical accounts, including accounts where payment details are stored. Set 2-factor authentication
            for account registered using an email address managed via this service, where possible.
            </dd>
          </div>

          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Liability</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            This service is provided to assist with managing low value email communication. We will accept no liability for 
            for loss (financial or otherwise) associated with using this service, including but not limited to lost communication.
            </dd>
          </div>

          <div class="relative">
            <dt>
              <InformationCircleIcon class="absolute h-6 w-6 text-green-700" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Continuity</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            Whilst we endeavour to keep this service running, we offer no warrantee on the longevity of this service.
            </dd>
          </div>
        </dl>
      </div>
    </div>
  </div>
</template>
<script>
// import { CheckIcon } from '@heroicons/vue/24/outline'
import { InformationCircleIcon } from '@heroicons/vue/24/outline'

export default {
  name: 'TermsSection',
  components: {
    // CheckIcon,
    InformationCircleIcon,
  }
};
</script>
