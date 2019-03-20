<template>
  <div id="NewOrganizationView">
    <CenteredPanel>
      <h1>Create an organization</h1>
      <p>This could be a company, a non-profit, a family, or even a D&D group.</p>

      <form @submit.prevent="onSubmit">
        <TextInput
          label="Title"
          v-model="title"
          :errMsg="titleErr"
          autofocus
        />
        <TextInput
          label="Your name"
          v-model="name"
          :errMsg="nameErr"
        />
        <Button type="submit" class-names="primary">Create</Button>
        <Button @click="cancel">Cancel</Button>
      </form>
    </CenteredPanel>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Api from '@/lib/api';

@Component
export default class extends Vue {
  title: string = '';
  titleErr: string = '';
  name: string = '';
  nameErr: string = '';

  cancel() {
    this.$router.go(-1);
  }

  onSubmit() {
    this.titleErr = '';
    this.nameErr = '';

    if (!this.title) {
      this.titleErr = 'cannot be blank';
    }

    if (!this.name) {
      this.nameErr = 'cannot be blank';
    }

    if (!this.titleErr && !this.nameErr) {
      Api.post('organizations', {
        organization: { title: this.title },
        organization_user: { name: this.name },
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
