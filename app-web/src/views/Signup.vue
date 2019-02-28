<template>
  <div class="registration">
    <h1>Sign Up</h1>
    <p>All we need from you are a valid email and a decent password.</p>

    <form @submit.prevent="onSubmit" class="form">
      <div class="form-group">
        <label for="email" class="form-label">Email</label>
        <input type="email"
          id="email"
          class="form-input"
          v-model="newUser.email"
        />
      </div>

      <div class="form-group">
        <label for="password" class="form-label">Password</label>
        <input type="password"
          id="password"
          class="form-input"
          v-model="newUser.password"
        />
      </div>

      <div class="form-group">
        <input type="password"
          id="password-confirmation"
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
import Vue from 'vue';
import Component from 'vue-class-component';
import Api from '@/lib/api';

interface NewUser {
  email: string;
  password: string;
  passwordConfirmation: string;
}

@Component
export default class Signup extends Vue {
  newUser: NewUser = {
    email: '',
    password: '',
    passwordConfirmation: '',
  };

  errors: string[] = [];

  onSubmit() {
    const user = this.newUser;
    this.errors = [];

    if (!user.email) {
      this.errors.push('Email cannot be blank');
    }

    if (!user.password) {
      this.errors.push('Password cannot be blank');
    }

    if (user.password !== user.passwordConfirmation) {
      this.errors.push('Password confirmation must match');
    }

    if (!this.errors.length) {
      Api.post('users', { user: { email: user.email, password: user.password } })
        .then((resp) => {
          localStorage.setItem('authenticated', 'true');
          this.$router.push({ name: 'chat' });
        })
        .catch((resp) => {
          debugger;
        });
    }
  }
};
</script>

<style scoped lang="scss">
</style>
