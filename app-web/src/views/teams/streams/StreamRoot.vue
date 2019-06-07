 <template>
   <router-view :key="$route.params.stream"/>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import { Next } from '@/types';
import { slugerize } from '@/lib/helpers';
import Repo from '@/lib/repo';


function verifyStream(to: Route, next: Next) {
  const { team, stream } = to.params;
  const repo = Repo.getTeam(team)!.getStream(stream);

  if (!repo) {
    const stream = Repo.getTeam(to.params.team)!.getDefaultStream()!.stream;
    const params = { ...to.params, stream: stream.key };

    return next({ name: 'streamRoot', params });
  }

  next();
}


@Component
export default class Stream extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    verifyStream(to, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    if (to.params.stream === from.params.stream) {
      to.name === 'streamRoot' ? next(false) : next();
      return;
    }

    verifyStream(to, next);
  }

  created() {
    const { team: teamKey, stream: streamKey } = this.$route.params;
    const stream = Repo.getTeam(teamKey)!.getStream(streamKey)!.stream;
    const newName = slugerize(stream.title);

    if (newName === this.$route.params.streamTitle) { return; }

    const params = { ...this.$route.params, streamTitle: newName };
    const toName = this.$route.name === 'streamRoot'
      ? 'stream'
      : this.$route.name;

    console.info('\tUpdating Stream name');
    this.$router.push({ name: toName, params });
  }
}
</script>
