<template>
  <div class="Signup">
    <p>To get started, all we need from you are a valid email and a decent password.</p>

    <form @submit.prevent="onSubmit">
      <TextInput label="Email" v-model="newUser.email" :errMsg="emailErr" autofocus/>
      <TextInput label="Password" v-model="newUser.password" :errMsg="passwordErr"/>
      <TextInput v-model="newUser.passwordConfirmation" placeholder="Confirm your password"/>
      <Button type="submit" class-names="primary">Sign Up</Button>
    </form>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Api from '@/lib/api';
import Auth from '@/lib/auth';

interface NewUser {
  email: string;
  password: string;
  passwordConfirmation: string;
}

@Component
export default class extends Vue {
  newUser: NewUser = {
    email: '',
    password: '',
    passwordConfirmation: '',
  };

  emailErr: string = '';
  passwordErr: string = '';

  onSubmit() {
    const user = this.newUser;

    this.emailErr = '';
    this.passwordErr = '';

    if (!user.email) {
      this.emailErr = 'cannot be blank';
    }

    if (!user.password) {
      this.passwordErr = 'cannot be blank';
    }

    if (user.password !== user.passwordConfirmation) {
      this.passwordErr = 'confirmation must match';
    }

    if (!this.emailErr && !this.passwordErr) {
      Api.post('users', { user: { email: user.email, password: user.password } })
        .then(resp => this.$emit('authenticated', resp))
        .catch(err => { debugger; alert(err); });
    }
  }
};
</script>
