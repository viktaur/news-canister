<script lang="ts">
    const displayProof = () => {
        const proofBox = document.querySelector('.proof-box');
        proofBox.style.display = 'block';
    }

    const hideProof = () => {
        const proofBox = document.querySelector('.proof-box');
        proofBox.style.display = 'none';
    }

    const changePos = (event: MouseEvent) => {
        const proofBox = document.querySelector('.proof-box');
        proofBox.style.top = `${event.clientY}px`;
        proofBox.style.left = `${event.clientX}px`;
    }

</script>

<div>
    <div class="post-list">
        <div class="post">
            <h3>@poppysmith737</h3>
            <p>Wow guys, you won’t believe this, but they’ve been <span class="proof-link" role="presentation" on:mousemove={changePos} on:mouseenter={displayProof} on:mouseleave={hideProof}>experimenting with alien life and humans at Area 51</span> for years!!!</p>
        </div>
        <div class="post">
            <h3>@jameshwright</h3>
            <p>According to documents leaked from the Department of Homeland security, <span class="proof-link" role="presentation" on:mousemove={changePos} on:mouseenter={displayProof} on:mouseleave={hideProof}>the government may have known about aliens</span> since a while back... We don’t know exactly who leaked this, but thanks to Silent Whistle we know the statement is legit!</p>
        </div>
        <div class="post">
            <h3>@M4G4tracy</h3>
            <p>I own a dog and two cats. After the recent news, I fear for my pets more than ever, until the point that I’m considering moving out of Springfield, forever... It’s very sad to see what our society has become.</p>
        </div>
    </div>
    <div class="proof-box" style="display: none">
        <div class="proof">
            <p>“evidence of alien-human interaction happening at Homey Airport during the years 2007 and 2011 was found by a top-secret investigation led by the Department of Defense.”</p>
            <div class="bottom-proof">
                <p><b>Valid proof</b>: 0x37FA...023A</p>
                <p><b>Source</b>: DHS</p>
            </div>
        </div>
    </div>
</div>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;0,14..32,300;0,14..32,400;0,14..32,700;1,14..32,100..900&display=swap');

    .bottom-proof {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .proof-box {
        background-color: #1044FF;
        padding: 20px;
        color: white;
        border-radius: 20px;
        position: absolute;
        top: 0;
        left: 0;
        width: 500px;
        /*height: 200px;*/
        z-index: 1;
    }

    .post-list {
        display: flex;
        flex-direction: column;
        gap: 20px;
        margin: 200px 400px;
    }

    .post {
        border-radius: 20px;
        box-shadow: 0 5px 15px #00000033;
        background-color: white;
        padding: 20px;
        font-weight: normal;
    }

    .proof-link {
        color: #1044FF;
        text-decoration: underline;
    }

    :global {
        body {
            background-color: #EEF1FF;
            font-family: Inter, sans-serif;
        }
    }
</style>