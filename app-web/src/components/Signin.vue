<template>
  <div class="Signin">
    <form @submit.prevent="onSubmit">
      <TextInput label="Email" v-model="user.email" :errMsg="emailErr" autofocus/>
      <TextInput label="Password" v-model="user.password" :errMsg="passwordErr"/>
      <Button type="submit" class-names="primary">Sign In</Button>
    </form>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import Component from 'vue-class-component';
import Api from '@/lib/api';
import Auth from '@/lib/auth';

interface User {
  email: string;
  password: string;
}

@Component
export default class extends Vue {
  user: User = {
    email: '',
    password: '',
  };

  emailErr: string = '';
  passwordErr: string = '';

  onSubmit() {
    const { email, password } = this.user;

    this.emailErr = '';
    this.passwordErr = '';

    if (!email) {
      this.emailErr = 'cannot be blank';
    }

    if (!password) {
      this.passwordErr = 'cannot be blank';
    }

    if (!this.emailErr && !this.passwordErr) {
      Api.post('session', { email, password })
        .then(resp => { debugger; this.$emit('authenticated', resp); })
        .catch(err => { debugger; alert(err); });
    }
  }
};
</script>
