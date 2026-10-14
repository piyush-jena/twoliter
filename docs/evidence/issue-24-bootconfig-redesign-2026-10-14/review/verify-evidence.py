"""Independent bounded read-only audit; output only in this review directory."""
from pathlib import Path
import json, hashlib, re, subprocess, struct, tarfile, csv
R=Path('/home/fedora/Repositories/symphony'); D=Path(__file__).resolve().parent; E=D.parent; V=E/'validation'
PR5='33dbf4bab8ea5810803f05cb7c5d184041be6cf0'; PR6='09380d356886842b8987b602ed2ef7daaefec4f3'; BR='5cfeacd31949b6f68c4ddae79978eab6c33bb9d1'
repo=R/'stages/24-uki-pcr-predictions/twoliter'
def read(p):return json.loads(Path(p).read_text())
def digest(p):
 with Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(repo),*args])
assert git('rev-parse','HEAD').decode().strip()==PR6
assert subprocess.run(['git','-C',str(repo),'merge-base','--is-ancestor',PR5,PR6]).returncode==0
for gram in ['bootconfig','grub']:
 f='tools/bootconfig/src/parsers/'+gram+'.pest'
 assert git('show',PR6+':'+f)==git('show','661f4abb:'+f)==git('show',PR5+':'+f)==git('show','515d9ae:'+f)
tools=read(V/'tools-verified.json')['binaries']
for item in tools.values():assert digest(item['path'])==item['sha256']
source_paths=git('ls-tree','-r','--name-only',PR6,'tools/bootconfig','tools/ukisys','tools/pcrsys','Cargo.toml','Cargo.lock').decode().splitlines()
source_bytes={p:git('show',PR6+':'+p) for p in source_paths}
summary=read(V/'summary.json'); assert summary['all_passed'] and len(summary['rows'])==10
assert read(V/'execution/outcome.json')['complete']
results=[]; artifacts=0; total_bytes=0; checked_commands=0
for row in summary['rows']:
 name=row['variant']; b=R/'.symphony/builds'/row['build_id']; d=V/'runs'/name/row['build_id']; live=d/'live'
 rc=read(b/'collection-receipt.json'); remote=rc['remote_result']; assert remote['exit_code']==0 and read(b/'result.json')['status']=='succeeded'
 assert digest(b/'input.tar')==rc['input_sha256']==remote['input_sha256'];assert digest(b/'build.log')==remote['log_sha256']
 for item in remote['files']:
  f=b/'artifacts'/item['path']; assert f.stat().st_size==item['size'] and digest(f)==item['sha256'],f
  artifacts+=1;total_bytes+=item['size']
 # Compare captured implementation, not merely the revision string, without extracting the archive.
 with tarfile.open(b/'input.tar') as tar:
  for f,expected in source_bytes.items():assert tar.extractfile('./twoliter/'+f).read()==expected,(name,f)
 source=read(d/'source-provenance.json'); assert source['sources']['twoliter']['revision']==PR6 and source['sources']['member']['revision']==BR
 assert all(not s['tracked_changes'] for s in source['sources'].values())
 assert source['dependencies'][0]['version']=='17.0.1-symphony.24.build-fc201212'
 assert read(d/'request.json')['twoliter_revision']==PR6 and read(d/'request.json')['variant']==name
 pred=read(d/'pcr-predictions.json'); original=Path(read(d/'artifact-verification.json')['image_directory'])/'pcr-predictions.json'
 assert (d/'pcr-predictions.json').read_bytes()==original.read_bytes()
 assert pred==read(d/'disk-predictions.stdout')==read(live/'ami-predictions.stdout')==read(live/'ami-predictions.json')
 a=read(live/'ami-predictions.request.json')['argv']; assert a==[tools['pcrsys']['path'],'ami','--ami-id',row['ami_id'],'--region','us-west-2']
 diskargv=read(d/'disk-predictions.request.json')['argv'];assert diskargv[0]==tools['pcrsys']['path'] and diskargv[1]=='disk'
 assert read(d/'registration-identity.stdout')['Account']=='533267423195'
 image=read(live/'ami-current.stdout')['Images'][0]; host=read(live/'instance-current.stdout')['Reservations'][0]['Instances'][0]
 assert image['ImageId']==host['ImageId']==row['ami_id'] and host['InstanceId']==row['instance_id']
 assert image['OwnerId']=='533267423195' and not image['Public'] and image['Architecture']=='x86_64' and image['TpmSupport']=='v2.0' and image['BootMode']=='uefi'
 assert image['BlockDeviceMappings']==read(d/'registered-image.stdout')['Images'][0]['BlockDeviceMappings']
 assert read(live/'ami-uefi.stdout')['UefiData']['Value'].strip()==(b/'artifacts/state/sbkeys/local/efi-vars.aws').read_text().strip()
 for f in d.glob('*.result.json'):assert read(f)['exit_code']==0,f
 for f in live.glob('*.result.json'):assert read(f)['exit_code']==0,f
 invs={}
 for f in live.glob('*.invocation.json'):
  key=f.name.removesuffix('.invocation.json'); inv=read(f);invs[key]=inv
  assert inv['InstanceId']==row['instance_id'] and inv['ResponseCode']==0 and inv['Status']=='Success'
  assert inv['StandardOutputContent'].encode()==(live/(key+'.stdout')).read_bytes()
  assert inv==read(live/(key+'-readback.stdout'))
  assert inv['CommandId'] in (live/(key+'.stderr')).read_text()
  request=read(live/(key+'.request.json')); assert request['argv'][0]=='bash' and request['argv'][1].endswith('/ssm-executor/scripts/sheltie-command.sh') and request['argv'][2:4]==[row['instance_id'],'us-west-2']
  checked_commands+=1
 def out(k):return invs[k]['StandardOutputContent']
 assert out('boot-id-before')==out('boot-id-after') and out('boot-id-before').strip()==row['boot_id']
 assert out('ready-before').strip()==out('ready-after').strip()=='active'
 assert invs['boot-id-before']['ExecutionStartDateTime']<invs['pcrs']['ExecutionStartDateTime']<invs['boot-id-after']['ExecutionStartDateTime']
 assert 'systemd-pcrphase-multi-user.service' in out('phase-units')
 for phase in ['configured','preconfigured','sysinit','multi-user']:
  assert re.search(r'systemd-pcrphase-'+phase+r'\.service\s+loaded active exited',out('phase-units'))
 assert 'State: running' in out('system-status') and '0 loaded units listed.' in out('failed-units')
 assert '1.66.0-symphony.24' in out('os-release') and name in out('os-release') and out('arch').strip()=='x86_64' and out('secureboot').strip()=='1'
 if 'fips' in name:assert out('fips').strip()=='1'
 if 'nvidia' in name:assert 'NVRM version:' in out('nvidia-driver') and out('nvidia-devices').strip()
 assert read(live/'cmdline.request.json')['argv'][-1]=='cat /proc/cmdline'
 cmdline=out('cmdline').encode(); expected=(d/'expected-cmdline.txt').read_bytes(); assert cmdline==expected and cmdline.endswith(b'\n')
 pcrcommand='tpm2_pcrread '+'+'.join(bank+':'+','.join(map(str,range(24))) for bank in ['sha256','sha384'])
 assert read(live/'pcrs.request.json')['argv'][-1]==pcrcommand
 banks={};bank=None
 for line in out('pcrs').splitlines():
  if line.strip() in ['sha256:','sha384:']:
   bank=line.strip()[:-1];assert bank not in banks;banks[bank]={};continue
  m=re.fullmatch(r'\s*(\d+)\s*:\s*0x([0-9A-Fa-f]+)\s*',line);assert m,line
  i=int(m[1]);assert i not in banks[bank]; banks[bank][i]=m[2].lower();assert len(m[2])=={'sha256':64,'sha384':96}[bank]
 assert set(banks)=={'sha256','sha384'} and all(set(v)==set(range(24)) for v in banks.values())
 indices={0,2,3,5,6,7,10,11,12,13,14,15}
 if row['boot_path']=='uki':indices|={4,9}
 assert set(map(int,pred['pcrs']))==indices
 states={};count=0
 for idx in sorted(indices):
  matches=[]
  for bank in ['sha256','sha384']:
   values=pred['pcrs'][str(idx)][bank]; allowed=[4] if idx==11 else range(len(values))
   matching={i for i in allowed if values[i].lower()==banks[bank][idx]}; assert matching,(name,bank,idx)
   matches.append(matching);count+=1
  states[idx]=sorted(set.intersection(*matches));assert states[idx]
 comparison=read(d/'comparison.json');assert states=={int(k):v for k,v in comparison['common_prediction_indices'].items()}
 csvrows=list(csv.DictReader((d/'pcr-comparisons.csv').open()));assert len(csvrows)==count
 for r in csvrows:assert r['actual']==banks[r['bank']][int(r['pcr'])] and r['match']=='True'
 assert len(comparison['unpredicted'])==48-count
 diagnostic={}
 if row['boot_path']=='grub':
  assert '9' not in pred['pcrs']
  for bank in banks:
   h=hashlib.new(bank,expected).digest();calc=hashlib.new(bank,bytes(len(h))+h).hexdigest();assert calc==banks[bank][9];diagnostic[bank]=calc
  args=read(d/'expected-cmdline.request.json')['argv'];assert args[0]==tools['command_line']['path']
  run=subprocess.run(args,capture_output=True,timeout=10);assert run.returncode==0 and run.stdout==expected
  assert 'Result=success' in out('cmdline-measurement') and 'ExecMainStatus=0' in out('cmdline-measurement')
 else:
  # Read the retained extracted signed EFI and decode its section table independently.
  args=read(d/'extract-fallback.request.json')['argv'];efi=Path(args[-1]).read_bytes();off=struct.unpack_from('<I',efi,0x3c)[0]
  countsects=struct.unpack_from('<H',efi,off+6)[0];optsize=struct.unpack_from('<H',efi,off+20)[0];found=[]
  for i in range(countsects):
   pos=off+24+optsize+40*i
   if efi[pos:pos+8].rstrip(b'\0')==b'.cmdline':
    vsize,_,rawsize,start=struct.unpack_from('<IIII',efi,pos+8);found.append(efi[start:start+min(vsize,rawsize)].rstrip(b'\0')+b'\n')
  assert found==[expected]
 results.append({'variant':name,'build_id':row['build_id'],'ami':row['ami_id'],'instance':row['instance_id'],'comparisons':count,'cmdline_bytes':len(expected),'common_states':states,'separate_pcr9_diagnostics':diagnostic,'verified':True})
 print(name,count,'PCR comparisons, exact bytes',len(expected),'verified',flush=True)
assert sum(r['comparisons'] for r in results)==256
result={'local_pr5':PR5,'local_pr6':PR6,'grammar_bytes_unchanged':True,'implementation_files_compared_per_image_input':len(source_paths),'artifact_count':artifacts,'artifact_bytes_hashed':total_bytes,'ssm_invocations_checked':checked_commands,'rows':results,'generated_comparisons':256,'separate_pcr9_diagnostics':12,'read_only':True}
(D/'independent-evidence-checks.json').write_text(json.dumps(result,indent=2)+'\n')
