<template>
  <div class="login">
    <h1>Sign In</h1>

    <form @submit.prevent="onSubmit" class="form">
      <div class="form-group">
        <label for="new-email" class="form-label">Email</label>
        <input type="email"
          id="new-email"
          class="form-input"
          v-model="user.email"
        />
      </div>

      <div class="form-group">
        <label for="new-password" class="form-label">Password</label>
        <input type="password"
          id="new-password"
          class="form-input"
          v-model="user.password"
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

interface User {
  email: string;
  password: string;
}

@Component
export default class Login extends Vue {
  user: User = {
    email: '',
    password: '',
  };

  errors: string[] = [];

  onSubmit() {
    const { email, password } = this.user;
    this.errors = [];

    if (!email) {
      this.errors.push('Email cannot be blank');
    }

    if (!password) {
      this.errors.push('Password cannot be blank');
    }

    if (!this.errors.length) {
      Api.post('login', { email, password })
        .then((resp) => {
          debugger;
        })
        .catch((resp) => {
          debugger;
        });
    }
  }
};
</script>
