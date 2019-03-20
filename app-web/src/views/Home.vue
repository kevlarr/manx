<template>
  <div id="HomeView">
    <!-- <img alt="Vue logo" src="../assets/logo.png"> -->
    <CenteredPanel>
      <h1>Welcome to Manx</h1>
      <p>Your home away from Slack.</p>
      <hr/>

      <div class="user-form">
        <component :is="activeComponent"/>
      </div>

      <p>{{ toggleMsg }}? <router-link :to="linkHref">{{ linkMsg }}</router-link></p>
    </CenteredPanel>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Signin from '@/components/Signin.vue';
import Signup from '@/components/Signup.vue';

@Component({
  components: { Signin, Signup },
})
export default class extends Vue {
  get onSignup() {
    return this.$route.query.signUp;
  }

  get activeComponent() {
    return this.onSignup ? 'Signup' : 'Signin';
  }

  get toggleMsg() {
    return this.onSignup ? 'Already a user' : 'Not a user yet';
  }

  get linkMsg() {
    return this.onSignup ? 'Sign in instead.' : 'Sign up for free!';
  }

  get linkHref() {
    return this.onSignup ? '' : '?signUp=true';
  }
}
</script>

<style lang='scss'>
#HomeView {
  .CenteredPanel > .user-form {
    margin-bottom: 35px;
  }
}
</style>
