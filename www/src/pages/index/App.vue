<template>
  <HeaderElement 
    @to_why="to('#whysection');"
    @to_get_started="to('#getstartedsection');"
    @to_pricing="to('#pricingsection');"
    @to_terms="to('#termssection');"
    @to_privacy="to('#privacysection');"
    @to_signin="to('#signsection'); sign='sign-in';"
    @to_signup="to('#signsection'); sign='sign-up';"
    />

  <BannerText>
    The service is free whilst in Beta version. Anticipated official launch: January 2023
  </BannerText>

  <HeaderSection src="/assets/img/mail.jpg">
    <template #header>Regain control of your mailbox</template>
    Stem the flow of emails from newsletters and services you subscribe and contact using an email masking service
    so that your mailbox remain relevant to your changing needs.
  </HeaderSection>

  <!-- Description Section -->
  <div class="bg-white border-t border-indigo-400 border-opacity-25" id="whysection" >
    <div class="py-12 sm:py-16 lg:py-20">
      <div class="px-6 max-w-3xl mx-auto">
        <div class="text-left text-lg text-gray-500">
          <h2 class="text-4xl font-bold text-gray-900">
          Email Privacy Revisited
          </h2>
          <p class="mt-8">
          Do you ever feel that your email is full of content from marketing, forums, newsletters
          that is no longer relevant, and is usually left unopened? That's how I felt.
          I wanted to regain control of my email inbox.
          </p>
          <p class="mt-6">
          I looked at the temporary email services available. These services are great when one just want 
          to register. But they do not quite meet my needs. I want to engage with
          the communication content, but need a way out when it becomes overwhelming and no longer relevant.
          A 10 minute email is just not quite right for me.
          </p>
          <p class="mt-6">
          I created <em>1-ml.com</em>, an email masking service, for this purpose. With it, 
          I can give unique email addresses when subscribing to newsletters, registering on websites,
          giving it to the shop assistant to have my shopping receipt emailed. These emails get forwarded on my 
          normal email address. But if/when I want to, I can mute them and any emails to the blocked addresses
          is stopped by the service. I feel in control and, as an added bonus, I feel more protected by
          using different passwords and email addresses - without the pain of opening multiple email accounts.
          </p>
          <p class="mt-6">
          This help me and I hope it can help you too.
          </p>
        </div>
      </div>
    </div>
  </div>

  <!-- How does it work -->
  <div class="bg-gray-100 border-t border-indigo-400 border-opacity-25" id="getstartedsection" >
    <div class="py-12 sm:py-16 lg:py-20">
      <div class="max-w-5xl mx-auto lg:grid lg:grid-cols-3 lg:gap-x-8">
        <div>
          <h3 class="ml-4 text-2xl font-bold text-gray-800">
          How does it work?
          </h3>
        </div>
        <dl class="ml-4 mt-12 lg:mt-0 lg:col-span-2 text-gray-500 lg:grid lg:gap-y-10">
          <HowToStep>
            <template #counter>1</template>
            <template #header>
            Register
            </template>
            <template #description>
            Just so we can create your account
            </template>
          </HowToStep>

          <HowToStep>
            <template #counter>2</template>
            <template #header>
            Verify your email address
            </template>
            <template #description>
            We don't want your emails to get lost or worst going to the wrong person
            </template>
          </HowToStep>

         <HowToStep>
           <template #counter>3</template>
           <template #header>
           Choose your prefix
           </template>
           <template #description>
           The prefix is unique to your account and any email sent to the prefix subdomain is yours.
           <div class="ml-8 mt-2 text-small text-gray-500">...@<span class="font-bold text-gray-600">prefix</span>.1-ml.com</div>
           </template>
         </HowToStep>

         <HowToStep>
           <template #counter>4</template>
           <template #header>
           All set
           </template>
           <template #description>
           Start giving unique email addresses using your prefix subdomain. There is no need to request
           email addresses in advance. As long as the email address uses your prefix subdomain, it goes
           to you.  
           <div class="ml-8 mt-2 text-small text-gray-500"><span class="font-bold text-gray-600">anything</span>@prefix.1-ml.com</div>
           <div class="ml-8 mt-0 text-small text-gray-500"><span class="font-bold text-gray-600">facebook</span>@prefix.1-ml.com</div>
           <div class="ml-8 mb-2 text-small text-gray-500"><span class="font-bold text-gray-600">linkedin</span>@prefix.1-ml.com</div>
           The email addresses used will appear in your account where you can control 
           whether to forward or block future communications individually on each address.
           </template>
         </HowToStep>

        </dl>
      </div>
    </div>
  </div>

  <!-- Pricing Section -->
  <div class="bg-white" id="pricingsection">
  <BannerText>
    The service is free whilst in Beta version. Anticipated official launch: January 2023
  </BannerText>

  <PricingSection 
    @view_terms="to('#termssection');"
    @start_trial="to('#signsection'); sign = 'sign-up';"
    />
  </div>

  <!-- Sign in / Sign up section -->
  <div class="bg-gray-100 border-t border-indigo-400 border-opacity-25" id="signsection">
    <div class="py-12 sm:py-16 lg:py-20">
      <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">

        <!-- Tab SignIn/SignUp -->
        <div class="sm:hidden">
          <label for="sign-tabs" class="sr-only">Select a tab</label>
          <select id="sign-tabs" v-model="sign" name="tabs" class="block w-full focus:ring-indigo-500 focus:border-indigo-500 border-gray-300 rounded-md">
            <option value="sign-in">Sign In</option>
            <option value="sign-up">Sign Up</option>
          </select>
        </div>
        <div class="hidden sm:block">
          <nav class="relative z-0 rounded-lg shadow flex divide-x divide-gray-200" aria-label="Tabs">
            <!-- Current: "text-gray-900", Default: "text-gray-500 hover:text-gray-700" -->
            <a href="#!" 
               @click="sign = 'sign-in'"
               class="group relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-sm font-medium text-center hover:bg-gray-50 focus:z-10" 
               :class="{'text-gray-900 rounded-l-lg': sign === 'sign-in', 'text-gray-500 hover:text-gray-700': sign !== 'sign-in'}"
               >
              <span>Sign In</span>
              <span aria-hidden="true" 
                    class="absolute inset-x-0 bottom-0 h-0.5"
                    :class="{'bg-indigo-500': sign === 'sign-in', 'bg-transparent': sign !== 'sign-in'}"
                    ></span>
            </a>

            <a href="#!"
               @click="sign = 'sign-up'"
               class="group relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-sm font-medium text-center hover:bg-gray-50 focus:z-10" 
               :class="{'text-gray-900 rounded-l-lg': sign === 'sign-up', 'text-gray-500 hover:text-gray-700': sign !== 'sign-up'}"
               >
              <span>Sign Up</span>
              <span aria-hidden="true" 
                    class="absolute inset-x-0 bottom-0 h-0.5"
                    :class="{'bg-indigo-500': sign === 'sign-up', 'bg-transparent': sign !== 'sign-up'}"
                    ></span>
            </a>
          </nav>
        </div>

        <!-- Sign In -->
        <div class="flex flex-col justify-center"
             v-if="sign === 'sign-in'">
          <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
            <div class="space-y-6">
              <InputComponent v-model="signin.username" label="Username" type="text" />
              <InputComponent v-model="signin.password" label="Password" type="password" />
        
              <div class="flex items-center justify-between">
                <div class="text-sm">
                  <a href="#!" @click="forgotPassword = true;" class="font-medium text-indigo-600 hover:text-indigo-500"> Forgot your password? </a>
                </div>
              </div>

              <div>
                <button @click="on_signin" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">Sign in</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Sign Up -->
        <div class="flex flex-col justify-center"
             v-if="sign === 'sign-up'">
          <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
            <div class="space-y-6">
              <InputComponent v-model="signup.username" label="Username" type="text" />
              <InputComponent v-model="signup.email"    label="Email address" type="email" />
              <div><InputComponentPassword v-model="signup.password" label="Password" /></div>
              <div>
                <button @click="on_signup" 
                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        :class="{'bg-indigo-600 hover:bg-indigo-700': signUpValid, 'bg-indigo-300': ! signUpValid}" 
                        :disabled="! signUpValid"
                        >Sign up</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Terms section -->
  <div class="bg-white border-t border-indigo-400 border-opacity-25" id="termssection" >
  <TermsSection />
  </div>

  <!-- Privacy section -->
  <div class="bg-gray-100 border-t border-indigo-400 border-opacity-25" id="privacysection" >
  <PrivacySection />
  </div>

  <FooterElement />

  <ForgotPasswordAlert
    @on_error="on_error($event.message, $event.error)"
    @on_loading="loading = $event;"
    @dismiss="forgotPassword = false;"
    @complete="forgotPassword = false;"
    v-show="forgotPassword == true" />
  <NewPasswordRequiredAlert
    :user="newPasswordRequired.user"
    :userAttributes="newPasswordRequired.userAttributes"
    :requiredAttributes="newPasswordRequired.requiredAttributes"
    @on_error="on_error($event.message, $event.error)"
    @dismiss="newPasswordRequired.needed = false;"
    @on_password_set="signin.password = $event; newPasswordRequired.needed = false;"
    v-show="newPasswordRequired.needed === true" />
  <SignUpConfirmAlert
    :email="signup.email"
    :username="signup.username"
    @dismiss="on_signup_confirmalert_dismiss"
    v-if="signUpConfirmAlert.show === true" />
  <VerificationAlert
    :code="verificationAlert.code"
    @code="verificationAlert.code = $event"
    :username="verificationAlert.username"
    @username="verificationAlert.username = $event"
    @on_error="on_error($event.message, $event.error)"
    @dismiss="verificationAlert.show = false"
    @on_verified="on_verified"
    v-if="verificationAlert.show === true" />
  <SnackAlert
    ref="snack_alert" />
  <LoadingAlert
    :message="loading"
    v-if="loading !== null" />
  <ErrorAlert 
    :message="error.message"
    :error="error.error"
    @dismiss="error = null;"
    v-if="error !== null" />
</template>

<script>
import HeaderElement from '@/components/HeaderElement.vue';
import HeaderSection from '@/components/HeaderSection.vue';
import FooterElement from '@/components/FooterElement.vue';
import PricingSection from '@/components/PricingSection.vue';
import InputComponent from '@/components/InputComponent.vue';
import InputComponentPassword from '@/components/InputComponentPassword.vue';
import ErrorAlert from '@/components/ErrorAlert.vue';
import LoadingAlert from '@/components/LoadingAlert.vue';
import HowToStep from './components/HowToStep.vue';
import TermsSection from './components/TermsSection.vue';
import NewPasswordRequiredAlert from './components/NewPasswordRequiredAlert.vue';
import ForgotPasswordAlert from './components/ForgotPasswordAlert.vue';
import { signUpCognitoUser, signInCognitoUser, getCurrentCognitoUser } from '@/js/cognito.js';
import PrivacySection from './components/PrivacySection.vue';
import SignUpConfirmAlert from './components/SignUpConfirmAlert.vue';
import VerificationAlert from './components/VerificationAlert.vue';
import SnackAlert from '@/components/SnackAlert.vue';
import BannerText from './components/BannerText.vue';

export default {
  name: 'App',
  components: {
    HeaderElement,
    HeaderSection,
    FooterElement,
    PricingSection,
    InputComponent,
    InputComponentPassword,
    ErrorAlert,
    LoadingAlert,
    HowToStep,
    TermsSection,
    NewPasswordRequiredAlert,
    ForgotPasswordAlert,
    PrivacySection,
    SignUpConfirmAlert,
    VerificationAlert,
    SnackAlert,
    BannerText,
  },
  data: function() {
    return {
      loading: null,
      error: null,
      sign: 'sign-in',
      signin: {
        username: null,
        password: null,
      },
      signup: {
        username: null,
        email: null,
        password: null,
      },
      newPasswordRequired: {
        needed: false,
        user: null,
        userAttributes: null,
        requiredAttributes: null,
      },
      currentUser: null,
      forgotPassword: false,
      signUpConfirmAlert: { show: false },
      verificationAlert: { show: false, code: null, username: null },
      verificationSuccessAlert: { show: false },
    };
  },
  mounted: function() {
    try {
      window.app = this;
      this.currentUser = getCurrentCognitoUser();
      let url = new URL(window.location.href);
      if (url.searchParams.has("activate")) {
        this.verificationAlert.show = true;
        this.verificationAlert.code = url.searchParams.get("activate");
      }
    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },
  computed: {
    signUpValid: function() { return this.signup.username && this.signup.email && this.signup.password; }
  },
  methods: {
    to: function(hash) {
      try {
        window.location.hash = hash;
      } catch(e) {
        this.on_error("Error in to", e);
      }
    },
    on_signin: function() {
      try {
        this.loading = "Signing in.";
        signInCognitoUser(this.signin.username, this.signin.password)
        .then((res) => { 
          this.loading = null; 
          if (res.type === 'onSuccess') {
            window.location.href = "/home.html"; 
          } else if (res.type === 'newPasswordRequired') {
            this.newPasswordRequired.needed = true;
            this.newPasswordRequired.user = res.user;
            this.newPasswordRequired.userAttributes = res.userAttributes;
            this.newPasswordRequired.requiredAttributes = res.requiredAttributes;
          } else {
            throw new Error(`Result type ${res.type} is not a supported value`);
          }
        })
        .catch((e) => {this.on_error(`Unable to sign-in user ${this.signin.username}`, e);});
      } catch (e) {
        this.on_error("Error in on_signin", e);
      }
    },
    on_signup: function() {
      try {
        this.loading = "Registering.";
        signUpCognitoUser(this.signup.username, this.signup.password, this.signup.email)
        .then(() => {
          this.loading = null; 
          this.signUpConfirmAlert.show = true;
        })
        .catch((e) => {this.on_error(`Unable to sign-up user ${this.signup.username}`, e);});
      } catch (e) {
        this.on_error("Error in on_signup", e);
      }
    },
    on_signup_confirmalert_dismiss: function() {
      try {
        this.signUpConfirmAlert.show = false;
        this.signup.username = null; 
        this.signup.password = null;
        this.signup.email = null;
        this.sign = 'sign-in';
      } catch (e) {
        this.on_error("Error in onsignup_confirmalert_dismiss", e);
      }
    },

    on_verified: function() {
      try {
        this.verificationAlert.show = false;
        this.$refs.snack_alert.display('Verification successfull. You are ready to sign in.');
        this.signin.username = this.verificationAlert.username;
        this.sign = 'sign-in';
        this.to('#signsection');
      } catch (e) {
        this.on_error("Error in on_verified", e);
      }
    },

    on_error: function(message, error) {
      this.loading = null;
      if (this.error === null) { this.error = {message, error}; }
    }
  }
}
</script>

