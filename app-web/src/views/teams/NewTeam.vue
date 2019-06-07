<template>
  <div id="NewTeamView">
    <CenteredPanel>
      <h1>Create a team</h1>
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
import Repo from '@/lib/repo';

@Component
export default class NewTeamView extends Vue {
  title: string = '';
  titleErr: string = '';
  name: string = '';
  nameErr: string = '';

  cancel() {
    this.$router.go(-1);
  }

  onSubmit() {
    const title = this.title;
    const username = this.name;

    this.titleErr = '';
    this.nameErr = '';

    if (!title) {
      this.titleErr = 'cannot be blank';
    }

    if (!username) {
      this.nameErr = 'cannot be blank';
    }

    if (!this.titleErr && !this.nameErr) {
      Repo.createTeam({ title, username })
        .then(repo => this.$router.push({
          name: 'teamRoot',
          params: { team: repo.team.key },
        }))
        .catch(err => alert(JSON.stringify(err)));
    }
  }
}
</script>
