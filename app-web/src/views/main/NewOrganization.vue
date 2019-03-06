<template>
  <div class="NewOrganization">
    <form @submit.prevent="onSubmit">
      <div>
        <label for="title">Title</label>
        <input id="title" v-model="newOrg.title" />
      </div>

      <div>
        <label for="orgUserName">Your Name</label>
        <input id="orgUserName" v-model="newOrgUser.name" />
      </div>

      <div>
        <label for="orgUserUsername">Username</label>
        <input id="orgUserUsername" v-model="newOrgUser.username" />
      </div>

      <div>
        <button type="submit">Create</button>
        <button @click="cancel">cancel</button>
      </div>
    </form>

    <ul>
      <li v-for="err in errors">{{ err }}</li>
    </ul>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Api from '@/lib/api';

interface NewOrg {
  title: string;
}

interface NewOrgUser {
  name: string;
  username: string;
}

@Component
export default class NewOrganization extends Vue {
  newOrg: NewOrg = {
    title: '',
  };
  newOrgUser: NewOrgUser = {
    name: '',
    username: '',
  };
  errors: string[] = [];

  cancel() {
    this.$router.go(-1);
  }

  onSubmit() {
    const { title } = this.newOrg;
    const { name, username } = this.newOrgUser;

    if (this.errors.length) {
      this.errors = [];
    }

    if (!title) {
      this.errors.push('Title cannot be blank');
    }

    if (!name) {
      this.errors.push('Name cannot be blank');
    }

    if (!username) {
      this.errors.push('Username cannot be blank');
    }

    if (!this.errors.length) {
      Api.post('organizations', {
        organization: { title },
        organization_user: { name, username },
      })
        .then((resp) => {
          debugger;
        })
        .catch((resp) => {
          debugger;
        });
    }
  }
}
</script>

<style scoped lang='scss'>
</style>
