<template>
  <div class="registration">
    <h1>Sign Up</h1>
    <p>All we need from you are a valid email and a decent password.</p>

    <form @submit.prevent="onSubmit" class="form">
      <div class="form-group">
        <label for="new-email" class="form-label">Email</label>
        <input type="email"
          id="new-email"
          class="form-input"
          v-model="newUser.email"
        />
      </div>

      <div class="form-group">
        <label for="new-password" class="form-label">Password</label>
        <input type="password"
          id="new-password"
          class="form-input"
          v-model="newUser.password"
        />
      </div>

      <div class="form-group">
        <input type="password"
          id="new-password-confirmation"
          class="form-input"
          placeholder="Confirm your password"
          v-model="newUser.passwordConfirmation"
        />
      </div>

      <div class="form-group">
        <button type="submit" class="button">Start Chatting</button>
      </div>
    </form>

    <ul class="form-errors">
      <li class="form-error" v-for="err in errors">{{ err }}</li>
    </ul>
  </div>
</template>

<script lang="ts">

import { Component, Vue } from 'vue-property-decorator';

interface NewUser {
  email: string;
  password: string;
  passwordConfirmation: string;
}

const emailRgx = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

@Component
export default class HelloWorld extends Vue {
  newUser: NewUser = {
    email: '',
    password: '',
    passwordConfirmation: '',
  };

  errors: string[] = [];

  onSubmit() {
    const user = this.newUser;
    this.errors = [];

    if (!emailRgx.test(user.email)) {
      this.errors.push('Email is invalid');
    }

    if (!user.password) {
      this.errors.push('Password cannot be blank');
    }

    if (user.password != user.passwordConfirmation) {
      this.errors.push('Password confirmation must match');
    }

    if (!this.errors.length) {
      alert('thanks for registering!');
    }
  }
};

</script>

<style scoped lang="scss">
</style>
